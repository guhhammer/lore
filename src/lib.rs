//! lore: a terminal list of every command you can type, grouped A to Z.
//!
//! Flow: `commands` finds the names → `app` holds the state → `interface::ui`
//! draws it → `interface::input` turns keys into actions → back to `app`. Modules are `pub` so the
//! tests in tests/ can use them.

use std::io::{self, IsTerminal};
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event;

pub mod app; // state: commands, search text, rows, selection
pub mod commands; // where the names come from (programs, shell, aliases)
pub mod compile_config; // hardcoded lists and paths
pub mod interface; // keys, rows, search, drawing
pub mod utils; // helpers: man pages, fitting text

use app::App;
use interface::input::{self, Action};
use interface::ui;

/// Runs lore until Esc or Ctrl+C, then gives the terminal back. Fails, with
/// a message for the user, when there is no terminal to draw on.
pub fn run() -> io::Result<()> {
    // piped, from cron, or `ssh host lore` without -t: there is no screen
    if !io::stdout().is_terminal() {
        return Err(io::Error::other(
            "needs a terminal; run it directly, not through a pipe (over ssh: ssh -t)",
        ));
    }
    let mut app = App::new(commands::collect());
    // full-screen mode, keys come raw; try_init returns an error instead of
    // panicking if the terminal cannot be set up
    let mut terminal = ratatui::try_init()?;
    let result = event_loop(&mut terminal, &mut app);
    ratatui::restore(); // always, even after an error, or the shell is left broken
    result
}

/// Draw, wait for an action, apply it; repeat until Quit.
fn event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    let mut draw = true;
    loop {
        if draw {
            terminal.draw(|frame| ui::draw(frame, app))?;
        }
        match input::next_action(app.mode)? {
            Action::Quit => return Ok(()),
            Action::Up => app.up(),
            Action::Down => app.down(),
            Action::OpenSearch => app.open_search(),
            Action::LeaveSearch => app.leave_search(),
            Action::OpenInfo => app.open_info(),
            Action::CloseInfo => app.close_info(),
            Action::LineUp => app.scroll_by(-1),
            Action::LineDown => app.scroll_by(1),
            Action::PageUp => app.scroll_by(-compile_config::PAGE_STEP),
            Action::PageDown => app.scroll_by(compile_config::PAGE_STEP),
            Action::Type(c) => app.type_char(c),
            Action::Erase => app.erase(),
            Action::Resize => {} // nothing to change: the loop redraws at the new size
        }
        // More keys already waiting (an arrow held down): apply them before
        // drawing, so a held key does not lag behind on a slow terminal.
        draw = !event::poll(Duration::ZERO)?;
    }
}
