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
        Start::Explain(line) => return explain(&line),
        Start::Complete(words) => return complete(&words),
        Start::Check(files) => return check_files(&files),
        Start::Completions(shell) => {
            let script = compile_config::COMPLETION_SCRIPTS
                .iter()
                .find(|(name, _)| *name == shell)
                .map(|(_, script)| *script)
                .ok_or_else(|| io::Error::other(compile_config::ERROR_COMPLETIONS_SHELL))?;
            print!("{script}");
            return Ok(());
        }
        Start::List => None,
        Start::Page(name) => Some(name),
    };
    let mut app = App::new(everything());
    app.palettes = interface::palette::all();
    app.chosen = interface::palette::at_start(&app.palettes);
    let history_off = settings::read(compile_config::SETTING_HISTORY).as_deref()
        == Some(compile_config::SETTING_OFF);
    let internet_off = settings::read(compile_config::SETTING_INTERNET).as_deref()
        == Some(compile_config::SETTING_OFF);
    if !internet_off {
        app.internet = utils::internet::Internet::start();
    }
    app.history = !history_off;
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

/// The list: every command on this machine, and lore's recipes.
fn everything() -> Vec<commands::Command> {
    let mut all = commands::collect();
    all.extend(info::recipes::commands());
    commands::merge(all)
}

/// `lore --print NAME`: the page as plain text, the same sections as on
/// screen, at the terminal's width, or `PRINT_WIDTH` in a pipe or a file.
/// Needs no terminal: it is for scripts, `grep` and `less`.
fn print_page(name: &str) -> io::Result<()> {
    let mut app = App::new(everything());
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

/// `lore check FILE…`: each file's errors and warnings with their line, or
/// "ok". Fails (exit 1) when any file has an error, so a program's build
/// can run it on the page it ships.
fn check_files(files: &[String]) -> io::Result<()> {
    use compile_config::{CHECK_OK, CHECK_UNREADABLE, ERROR_CHECK_FAILED, PAGE_MAX_BYTES};
    use std::io::Read;
    let mut failed = 0;
    for file in files {
        let path = std::path::Path::new(file);
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.strip_suffix(".md").unwrap_or(n))
            .unwrap_or_default();
        let mut bytes = Vec::new();
        let read = std::fs::File::open(path)
            .and_then(|f| f.take(PAGE_MAX_BYTES as u64 + 1).read_to_end(&mut bytes));
        if read.is_err() {
            println!("{}", CHECK_UNREADABLE.replace("{file}", file));
            failed += 1;
            continue;
        }
        let findings = info::check::check(name, &bytes);
        if findings.is_empty() {
            println!("{}", CHECK_OK.replace("{file}", file));
        }
        for finding in &findings {
            let kind = if finding.error { "error" } else { "warning" };
            println!("{file}:{}: {kind}: {}", finding.line, finding.text);
        }
        if !info::check::passes(&findings) {
            failed += 1;
        }
    }
    if failed > 0 {
        return Err(io::Error::other(
            ERROR_CHECK_FAILED.replace("{count}", &failed.to_string()),
        ));
    }
    Ok(())
}

/// `lore --complete WORDS`: one candidate a line, `word<TAB>what it is`,
/// for the completion scripts. Never fails loudly: a completion that cannot
/// be worked out is just none.
fn complete(words: &[String]) -> io::Result<()> {
    use info::{content, recipes};
    let all = everything();
    let names: Vec<String> = all.iter().map(|c| c.name.clone()).collect();
    let describe = |name: &str| {
        let summary = all
            .iter()
            .find(|c| c.name == name)
            .and_then(recipes::content_of)
            .and_then(|page| page.summary.into_iter().next())
            .unwrap_or_default();
        summary.split(". ").next().unwrap_or_default().to_owned()
    };
    let options_of = |program: &str| {
        utils::manual::page(
            &content::page_key(program),
            compile_config::EXPLAIN_MAN_WIDTH,
        )
        .map(|lines| info::explain::options_in_man(&lines))
        .unwrap_or_default()
    };
    let found = interface::complete::candidates(
        words,
        &interface::complete::Sources {
            names: &names,
            describe: &describe,
            options_of: &options_of,
        },
    );
    let mut out = String::new();
    for (word, what) in found {
        // a tab or a newline in a description would break the line format
        let what: String = what
            .chars()
            .map(|c| if c.is_control() { ' ' } else { c })
            .collect();
        let what = utils::text::fit(
            what.trim_end_matches('.'),
            compile_config::COMPLETE_DESCRIBE_WIDTH,
        );
        out.push_str(&format!("{word}\t{what}\n"));
    }
    match io::stdout().lock().write_all(out.as_bytes()) {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    }
}

/// `lore explain LINE`: the line, then each word and what it means, the
/// meanings wrapped at the terminal's width (or `PRINT_WIDTH`).
fn explain(line: &str) -> io::Result<()> {
    use utils::text;
    let manual = |name: &str| utils::manual::page(name, compile_config::EXPLAIN_MAN_WIDTH);
    let summary = |name: &str| {
        info::content::find(name)
            .and_then(|page| page.summary.into_iter().next())
            .or_else(|| utils::manual::whatis(&info::content::page_key(name)))
    };
    let rows = info::explain::explain(
        line,
        &info::explain::Lookup {
            manual: &manual,
            summary: &summary,
        },
    );
    let width = usize::from(if io::stdout().is_terminal() {
        ratatui::crossterm::terminal::size().map_or(compile_config::PRINT_WIDTH, |(cols, _)| cols)
    } else {
        compile_config::PRINT_WIDTH
    });
    let column = rows
        .iter()
        .map(|(word, _)| text::width(word))
        .filter(|&w| w <= compile_config::EXPLAIN_WORD_WIDTH)
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    out.push_str(&format!("{line}\n\n"));
    for (word, meaning) in rows {
        let pad = " ".repeat(column + 4);
        let room = width.saturating_sub(column + 4).max(20);
        let mut wrapped = text::wrap(&meaning, room).into_iter();
        if text::width(&word) > column {
            out.push_str(&format!("  {word}\n")); // too long for the column: a line of its own
        } else if let Some(first) = wrapped.next() {
            out.push_str(&format!(
                "  {word}{}  {first}\n",
                " ".repeat(column - text::width(&word))
            ));
        }
        for more in wrapped {
            out.push_str(&format!("{pad}{more}\n"));
        }
    }
    match io::stdout().lock().write_all(out.as_bytes()) {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    }
}

/// Saves the recent pages for the next runs; if they cannot be saved, they
/// still hold for this one.
fn keep_recent(app: &App) {
    let _ = settings::write_list(compile_config::RECENT_FILE, &app.recent);
}

/// Enter on a setting of the about screen: the palette opens its picker;
/// the internet and the history switch on or off, at once and for the next
/// runs. One that cannot be saved still holds for this run, and the key bar
/// says why.
fn change_setting(app: &mut App) {
    use app::Setting;
    use compile_config::{SETTING_HISTORY, SETTING_INTERNET, SETTING_OFF, SETTING_ON};
    let word = |on: bool| if on { SETTING_ON } else { SETTING_OFF };
    let saved = match app.chosen_setting() {
        Setting::Colours => {
            app.open_palettes();
            return;
        }
        Setting::Internet => {
            let on = !app.internet.enabled();
            app.internet.set_enabled(on);
            if on {
                app.ask_internet();
            }
            settings::write(SETTING_INTERNET, word(on))
        }
        Setting::History => {
            let on = !app.history;
            let uses = if on {
                utils::history::read()
            } else {
                Default::default()
            };
            app.set_history(on, uses);
            settings::write(SETTING_HISTORY, word(on))
        }
    };
    if let Err(error) = saved {
        app.note = Some(compile_config::SETTING_NOT_SAVED.replace("{why}", &error.to_string()));
    }
}

/// Draw, wait for an action, apply it; repeat until Quit.
fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    let mut draw = true;
    loop {
        if draw {
            terminal.draw(|frame| ui::draw(frame, app))?;
        }
        let internet = std::sync::Arc::clone(&app.internet);
        let action = input::next_action(app.mode, &|| internet.take_changed())?;
        if action != Action::Redraw {
            app.note = None; // a note on the key bar lasts until the next key
        }
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
            Action::CloseInfo => app.close_info(),
            Action::NextLetter => app.next_letter(),
            Action::PreviousLetter => app.previous_letter(),
            Action::NextTopic => app.step_topic(1),
            Action::PreviousTopic => app.step_topic(-1),
            Action::NextSection => app.next_section(),
            Action::PreviousSection => app.previous_section(),
            Action::LineUp => app.scroll_by(-1),
            Action::LineDown => app.scroll_by(1),
            Action::PageUp => app.scroll_by(-compile_config::PAGE_STEP),
            Action::PageDown => app.scroll_by(compile_config::PAGE_STEP),
            Action::Type(c) => app.type_char(c),
            Action::Erase => app.erase(),
            Action::OpenAbout => app.open_about(),
            Action::CloseAbout => app.close_about(),
            Action::PreviousSetting => app.step_setting(-1),
            Action::NextSetting => app.step_setting(1),
            Action::ChangeSetting => change_setting(app),
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
            // nothing to change: the loop draws again (a new size, news)
            Action::Resize | Action::Redraw => {}
        }
        // More keys already waiting (an arrow held down): apply them before
        // drawing, so a held key does not lag behind on a slow terminal.
        draw = !event::poll(Duration::ZERO)?;
    }
}
