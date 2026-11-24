//! lore: a terminal list of every command you can type, grouped A to Z.
//!
//! Flow: `commands` finds the names → `app` holds the state → `interface::ui`
//! draws it → `interface::input` turns keys into actions → back to `app`.
//! Modules are `pub` so the tests in tests/ can use them.

use std::io::{self, IsTerminal, Write};
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event;

pub mod app; // state: commands, search text, rows, selection
pub mod commands; // where the names come from (programs, shell, aliases)
pub mod compile_config; // hardcoded lists and paths
pub mod info; // the page beside the list: content, version, links, manual
pub mod interface; // keys, rows, search, drawing
pub mod settings; // the settings file, kept between runs
pub mod utils; // helpers: man pages, fitting text

use app::App;
use interface::args::{self, Start};
use interface::input::{self, Action};
use interface::ui;

/// Runs lore as the command line asks (`lore`, `lore NAME`, `--help`,
/// `--version`) until Esc or Ctrl+C, then gives the terminal back. Fails,
/// with a message for the user, on a bad argument, a name with nothing to
/// show, or no terminal to draw on.
pub fn run() -> io::Result<()> {
    let start = args::parse(std::env::args().skip(1)).map_err(io::Error::other)?;
    let page = match start {
        Start::Help => {
            println!("{}", compile_config::HELP);
            return Ok(());
        }
        Start::Version => {
            println!("lore {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Start::Print(name) => return print_page(&name),
        Start::List => None,
        Start::Page(name) => Some(name),
    };
    let mut app = App::new(commands::collect());
    app.palettes = interface::palette::all();
    app.chosen = interface::palette::at_start(&app.palettes);
    let history_off = settings::read(compile_config::SETTING_HISTORY).as_deref()
        == Some(compile_config::SETTING_OFF);
    app.set_personal(
        settings::read_list(compile_config::BOOKMARKS_FILE),
        settings::read_list(compile_config::RECENT_FILE),
        if history_off {
            Default::default()
        } else {
            utils::history::read()
        },
    );
    if let Some(name) = page {
        // checked first, as man does, and before the screen is taken, so the
        // message lands in the shell
        if !app.open_direct(&name) {
            return Err(io::Error::other(
                compile_config::ERROR_NOTHING_NAMED.replace("{name}", &name),
            ));
        }
        keep_recent(&app);
    }
    // piped, from cron, or `ssh host lore` without -t: there is no screen
    if !io::stdout().is_terminal() {
        return Err(io::Error::other(compile_config::ERROR_NO_TERMINAL));
    }
    // full-screen mode, keys come raw; try_init returns an error instead of
    // panicking if the terminal cannot be set up
    let mut terminal = ratatui::try_init()?;
    let result = event_loop(&mut terminal, &mut app);
    ratatui::restore(); // always, even after an error, or the shell is left broken
    result
}

/// `lore --print NAME`: the page as plain text, the same sections as on
/// screen, at the terminal's width, or `PRINT_WIDTH` in a pipe or a file.
/// Needs no terminal: it is for scripts, `grep` and `less`.
fn print_page(name: &str) -> io::Result<()> {
    let mut app = App::new(commands::collect());
    if !app.open_direct(name) {
        return Err(io::Error::other(
            compile_config::ERROR_NOTHING_NAMED.replace("{name}", name),
        ));
    }
    let width = if io::stdout().is_terminal() {
        ratatui::crossterm::terminal::size().map_or(compile_config::PRINT_WIDTH, |(cols, _)| cols)
    } else {
        compile_config::PRINT_WIDTH
    };
    app.load_page(width);
    let lines = app.page.map(|page| page.lines).unwrap_or_default();
    let mut out = io::stdout().lock();
    for line in lines {
        match writeln!(out, "{}", line.text.trim_end()) {
            // `lore --print tar | head`: the reader has gone, which is fine
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => return Ok(()),
            result => result?,
        }
    }
    Ok(())
}

/// Saves the recent pages for the next runs; if they cannot be saved, they
/// still hold for this one.
fn keep_recent(app: &App) {
    let _ = settings::write_list(compile_config::RECENT_FILE, &app.recent);
}

/// Draw, wait for an action, apply it; repeat until Quit.
fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    let mut draw = true;
    loop {
        if draw {
            terminal.draw(|frame| ui::draw(frame, app))?;
        }
        let action = input::next_action(app.mode)?;
        app.note = None; // a note on the key bar lasts until the next key
        match action {
            Action::Quit => return Ok(()),
            Action::Up => app.up(),
            Action::Down => app.down(),
            Action::OpenSearch => app.open_search(),
            Action::LeaveSearch => app.leave_search(),
            Action::OpenInfo => {
                app.open_info();
                keep_recent(app);
            }
            Action::ToggleBookmark => {
                if app.toggle_bookmark() {
                    // a list that cannot be saved still holds for this run
                    let _ = settings::write_list(compile_config::BOOKMARKS_FILE, &app.bookmarks);
                }
            }
            // `lore NAME` was a question about NAME: answered, lore is done
            Action::CloseInfo if app.direct => return Ok(()),
            Action::CloseInfo => app.close_info(),
            Action::LineUp => app.scroll_by(-1),
            Action::LineDown => app.scroll_by(1),
            Action::PageUp => app.scroll_by(-compile_config::PAGE_STEP),
            Action::PageDown => app.scroll_by(compile_config::PAGE_STEP),
            Action::Type(c) => app.type_char(c),
            Action::Erase => app.erase(),
            Action::OpenAbout => app.open_about(),
            Action::CloseAbout => app.close_about(),
            Action::OpenCopy => app.open_copy(),
            Action::PreviousExample => app.step_example(-1),
            Action::NextExample => app.step_example(1),
            Action::CloseCopy => app.close_copy(None),
            Action::CopyExample => {
                let example = app.chosen_example();
                // the terminal does the copying; one that cannot just ignores it
                let sent = example.filter(|text| utils::clipboard::copy(text).is_ok());
                app.close_copy(sent.as_deref());
            }
            Action::OpenPalettes => app.open_palettes(),
            Action::PreviousPalette => app.step_palette(-1),
            Action::NextPalette => app.step_palette(1),
            Action::CancelPalette => app.cancel_palettes(),
            // saved for the next runs; if it cannot be, the picker stays open
            // and says so, and Esc or Enter again are the user's choice
            Action::KeepPalette => {
                match settings::write(compile_config::SETTING_PALETTE, &app.palette().name) {
                    Ok(_) => app.close_palettes(),
                    Err(error) => app.palette_failed(&error.to_string()),
                }
            }
            Action::Resize => {} // nothing to change: the loop redraws at the new size
        }
        // More keys already waiting (an arrow held down): apply them before
        // drawing, so a held key does not lag behind on a slow terminal.
        draw = !event::poll(Duration::ZERO)?;
    }
}
