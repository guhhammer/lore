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
pub mod configs; // hardcoded lists and paths
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
            println!("{}", configs::HELP);
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
        Start::UpdatePages => {
            // asked for by name, so it runs even with Internet off: the
            // setting is about what lore does on its own
            let done =
                info::updates::run(&mut |line| eprintln!("{line}")).map_err(io::Error::other)?;
            println!("{done}");
            return Ok(());
        }
        Start::Completions(shell) => {
            let script = configs::COMPLETION_SCRIPTS
                .iter()
                .find(|(name, _)| *name == shell)
                .map(|(_, script)| *script)
                .ok_or_else(|| io::Error::other(configs::ERROR_COMPLETIONS_SHELL))?;
            print!("{script}");
            return Ok(());
        }
        Start::List => None,
        Start::Page(name) => Some(name),
    };
    // before the list is read, so its pages' summaries are in that language
    let reading = saved_reading();
    let mut app = App::new(everything());
    app.palettes = interface::palette::all();
    app.chosen = interface::palette::at_start(&app.palettes);
    let history_off =
        settings::read(configs::SETTING_HISTORY).as_deref() == Some(configs::SETTING_OFF);
    let internet_off =
        settings::read(configs::SETTING_INTERNET).as_deref() == Some(configs::SETTING_OFF);
    if !internet_off {
        app.internet = utils::internet::Internet::start();
        info::beacon::refresh(); // a newer table for the next start, if due
    }
    app.set_beacon(info::beacon::load());
    app.history = !history_off;
    app.reading = reading;
    // `favourite = Python` (or `none`)
    let favourite = settings::read(configs::SETTING_FAVOURITE)
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty() && name != configs::FAVOURITE_NONE);
    app.set_favourite(favourite);
    // `scroll = 15`: one of the choices, else the default
    app.fast_scroll = settings::read(configs::SETTING_FAST_SCROLL)
        .and_then(|lines| lines.trim().parse().ok())
        .filter(|lines| configs::FAST_SCROLL_CHOICES.contains(lines))
        .unwrap_or(configs::FAST_SCROLL_DEFAULT);
    app.set_personal(
        settings::read_list(configs::BOOKMARKS_FILE),
        settings::read_list(configs::RECENT_FILE),
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
                configs::ERROR_NOTHING_NAMED.replace("{name}", &name),
            ));
        }
        keep_recent(&app);
    }
    // piped, from cron, or `ssh host lore` without -t: there is no screen
    if !io::stdout().is_terminal() {
        return Err(io::Error::other(configs::ERROR_NO_TERMINAL));
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
    let taken = all.iter().map(|c| c.name.clone()).collect();
    all.extend(info::languages::commands(&taken));
    commands::merge(all)
}

/// `lore --print NAME`: the page as plain text, the same sections as on
/// screen, at the terminal's width, or `PRINT_WIDTH` in a pipe or a file.
/// Needs no terminal: it is for scripts, `grep` and `less`.
fn print_page(name: &str) -> io::Result<()> {
    saved_reading(); // Pages in, as on screen
    let mut app = App::new(everything());
    if !app.open_direct(name) {
        return Err(io::Error::other(
            configs::ERROR_NOTHING_NAMED.replace("{name}", name),
        ));
    }
    let width = if io::stdout().is_terminal() {
        ratatui::crossterm::terminal::size().map_or(configs::PRINT_WIDTH, |(cols, _)| cols)
    } else {
        configs::PRINT_WIDTH
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
    use configs::{CHECK_OK, CHECK_UNREADABLE, ERROR_CHECK_FAILED, PAGE_MAX_BYTES};
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
        utils::manual::page(&content::page_key(program), configs::EXPLAIN_MAN_WIDTH)
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
        let what = utils::text::fit(what.trim_end_matches('.'), configs::COMPLETE_DESCRIBE_WIDTH);
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
    let manual = |name: &str| utils::manual::page(name, configs::EXPLAIN_MAN_WIDTH);
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
        ratatui::crossterm::terminal::size().map_or(configs::PRINT_WIDTH, |(cols, _)| cols)
    } else {
        configs::PRINT_WIDTH
    });
    let column = rows
        .iter()
        .map(|(word, _)| text::width(word))
        .filter(|&w| w <= configs::EXPLAIN_WORD_WIDTH)
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
    let _ = settings::write_list(configs::RECENT_FILE, &app.recent);
}

/// Enter on a setting of the about screen: the palette opens its picker;
/// the internet and the history switch on or off, at once and for the next
/// runs. One that cannot be saved still holds for this run, and the key bar
/// says why.
fn change_setting(app: &mut App) {
    use app::Setting;
    use configs::{SETTING_HISTORY, SETTING_INTERNET, SETTING_OFF, SETTING_ON};
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
        Setting::Reset => {
            if app.reset_armed {
                reset_all(app);
            } else {
                app.reset_armed = true; // asks first: Enter again does it
            }
            return;
        }
        Setting::Reading => {
            let current = app
                .reading
                .clone()
                .unwrap_or_else(|| configs::READING_SYSTEM.to_owned());
            app.open_choice(Setting::Reading, reading_choices(), &current);
            return;
        }
        Setting::Favourite => {
            let mut languages: Vec<String> = app
                .commands
                .iter()
                .filter(|c| c.kind == commands::Kind::Language)
                .map(|c| c.name.clone())
                .collect();
            languages.sort_by_key(|name| name.to_lowercase());
            let mut names = vec![configs::FAVOURITE_NONE.to_owned()];
            names.extend(languages);
            let current = app
                .favourite
                .clone()
                .unwrap_or_else(|| configs::FAVOURITE_NONE.to_owned());
            app.open_choice(Setting::Favourite, names, &current);
            return;
        }
        Setting::FastScroll => {
            app.next_fast_scroll();
            settings::write(configs::SETTING_FAST_SCROLL, &app.fast_scroll.to_string())
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
        app.note = Some(configs::SETTING_NOT_SAVED.replace("{why}", &error.to_string()));
    }
}

/// Pages in's choices: `system`, English, then every language a
/// translation is in (the command pages' and the language pages'), sorted.
pub fn reading_choices() -> Vec<String> {
    let mut codes: Vec<&str> = info::content::translated_codes();
    codes.extend(info::languages::translated_codes());
    codes.sort_unstable();
    codes.dedup();
    let mut choices = vec![
        configs::READING_SYSTEM.to_owned(),
        configs::READING_ENGLISH.to_owned(),
    ];
    choices.extend(codes.into_iter().map(str::to_owned));
    choices
}

/// Pages in from the settings file (`reading = pt_BR`; `system` or none:
/// LANG decides), applied for this run.
fn saved_reading() -> Option<String> {
    let reading = settings::read(configs::SETTING_READING)
        .map(|code| code.trim().to_owned())
        .filter(|code| !code.is_empty() && code != configs::READING_SYSTEM);
    if let Ok(mut chosen) = configs::READING_LANGUAGE.write() {
        chosen.clone_from(&reading);
    }
    reading
}

/// Applies Pages in for this run: a code, or None for `system` (LANG).
pub fn set_reading(app: &mut App, code: Option<String>) {
    if let Ok(mut chosen) = configs::READING_LANGUAGE.write() {
        chosen.clone_from(&code);
    }
    app.reading = code;
    app.page = None; // laid out again, in the new language, when drawn
    app.reload_docs(); // and the search reads that language's pages
}

/// Reset all, asked twice: every saved thing erased (settings file, lists,
/// downloads, caches) and every setting back to its default, now.
fn reset_all(app: &mut App) {
    use configs::{BEACON_CACHE_FILE, INTERNET_CACHE_DIR};
    app.reset_armed = false;
    let mut extra: Vec<std::path::PathBuf> = info::updates::folder().into_iter().collect();
    extra.extend(info::docs::folder()); // the downloaded docs
    app.close_docset();
    if let Some(cache) = utils::internet::cache_home() {
        extra.push(cache.join(INTERNET_CACHE_DIR));
        extra.push(cache.join(BEACON_CACHE_FILE));
    }
    let failed = settings::erase_all(&extra);
    // the defaults, as a first start would have them
    app.chosen = 0; // `terminal`, first of the palettes (build.rs puts it there)
    app.internet.set_enabled(true);
    app.set_history(true, utils::history::read());
    app.fast_scroll = configs::FAST_SCROLL_DEFAULT;
    set_reading(app, None);
    app.set_personal(Vec::new(), Vec::new(), app.uses.clone());
    app.set_favourite(None);
    app.set_beacon(info::beacon::load());
    app.note = Some(match failed.first() {
        None => configs::RESET_DONE.to_owned(),
        Some(error) => configs::RESET_FAILED.replace("{why}", &error.to_string()),
    });
}

/// `d` and `x`: the docs set in view (the one selected, the one open, or
/// the one whose page is shown) downloaded in the background, or deleted.
fn docs_key(app: &mut App, action: Action) {
    use commands::Kind;
    use configs::{DOCS_DELETED, DOCS_NOT_DELETED, DOCS_STARTED};
    let in_view = match app.mode {
        app::Mode::Info => app.page.as_ref().map(|page| page.command.clone()),
        _ => app.selected().cloned(),
    };
    let slug = match in_view {
        Some(command) if matches!(command.kind, Kind::Docset | Kind::DocEntry) => command
            .runs
            .and_then(|runs| runs.split('\t').next().map(str::to_owned)),
        _ => None,
    };
    let Some(set) = slug.and_then(|slug| app.docsets.iter().find(|s| s.slug == slug).cloned())
    else {
        return; // not a docs set: d and x do nothing here
    };
    let name = set.title();
    if action == Action::Download {
        app.downloads.start(set);
        app.note = Some(DOCS_STARTED.replace("{name}", &name));
    } else {
        if app
            .docset
            .as_ref()
            .is_some_and(|(open, _)| open.slug == set.slug)
        {
            app.close_docset();
        }
        app.note = Some(match info::docs::delete(&set.slug) {
            Ok(()) => DOCS_DELETED.replace("{name}", &name),
            Err(why) => DOCS_NOT_DELETED
                .replace("{name}", &name)
                .replace("{why}", &why.to_string()),
        });
    }
    app.refresh_rows();
}

/// Keeps what was chosen in a setting's picker, for this run and the next.
fn keep_choice(app: &mut App) {
    use app::Setting;
    let Some((setting, item)) = app.close_choice(true) else {
        return;
    };
    let saved = match setting {
        Setting::Reading => {
            let code = (item != configs::READING_SYSTEM).then(|| item.clone());
            set_reading(app, code);
            settings::write(configs::SETTING_READING, &item)
        }
        _ => {
            let name = (item != configs::FAVOURITE_NONE).then(|| item.clone());
            app.set_favourite(name);
            settings::write(configs::SETTING_FAVOURITE, &item)
        }
    };
    if let Err(error) = saved {
        app.note = Some(configs::SETTING_NOT_SAVED.replace("{why}", &error.to_string()));
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
        let downloads = std::sync::Arc::clone(&app.downloads);
        let action = input::next_action(app.mode, &|| {
            // `|` (not `||`): both are read, so neither change is lost
            internet.take_changed() | downloads.take_changed()
        })?;
        if action != Action::Redraw {
            app.note = None; // a note on the key bar lasts until the next key
        }
        match action {
            Action::Quit => return Ok(()),
            // Esc: out of an open docs set first, then out of lore
            Action::Escape => {
                if !app.close_docset() {
                    return Ok(());
                }
            }
            Action::Download | Action::Delete => docs_key(app, action),
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
                    let _ = settings::write_list(configs::BOOKMARKS_FILE, &app.bookmarks);
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
            // on a page they scroll it; in the list (and the search's
            // results) they move the selection
            Action::PageUp | Action::PageDown | Action::FastUp | Action::FastDown => {
                let lines = match action {
                    Action::PageUp | Action::PageDown => configs::PAGE_STEP,
                    _ => isize::try_from(app.fast_scroll).unwrap_or(isize::MAX),
                };
                let lines = if matches!(action, Action::PageUp | Action::FastUp) {
                    -lines
                } else {
                    lines
                };
                match app.mode {
                    app::Mode::Info => app.scroll_by(lines),
                    // the about screen on a small window (ui.rs clamps it)
                    app::Mode::About => {
                        app.about_scroll = app.about_scroll.saturating_add_signed(lines);
                    }
                    _ => app.move_by(lines),
                }
            }
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
            Action::StepChoice(steps) => app.step_choice(steps),
            Action::JumpChoice(letter) => app.jump_choice(letter),
            Action::KeepChoice => keep_choice(app),
            Action::CancelChoice => {
                app.close_choice(false);
            }
            Action::PreviousPalette => app.step_palette(-1),
            Action::NextPalette => app.step_palette(1),
            Action::CancelPalette => app.cancel_palettes(),
            // saved for the next runs; if it cannot be, the picker stays open
            // and says so, and Esc or Enter again are the user's choice
            Action::KeepPalette => {
                match settings::write(configs::SETTING_PALETTE, &app.palette().name) {
                    Ok(_) => app.close_palettes(),
                    Err(error) => app.palette_failed(&error.to_string()),
                }
            }
            // nothing to change: the loop draws again (a new size, news)
            Action::Resize => {}
            // a download ended: the docs topic shows its new state
            Action::Redraw => app.refresh_rows(),
        }
        // More keys already waiting (an arrow held down): apply them before
        // drawing, so a held key does not lag behind on a slow terminal.
        draw = !event::poll(Duration::ZERO)?;
    }
}
