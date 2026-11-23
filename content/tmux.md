# tmux

Terminal multiplexer. It allows multiple sessions with windows, panes, and more.

## Usage
- `tmux` : start a new session
- `tmux new -s name` : start a new named session
- `tmux ls` : list existing sessions
- `tmux a -t name` : attach to a named session (target can be omitted to attach to the most recently used session)
- `<Ctrl b><d>` : detach from the current session (inside a tmux session)
- `<Ctrl b><c>` : create a new window (inside a tmux session)
- `<Ctrl b><w>` : switch between sessions and windows (inside a tmux session)
- `tmux kill-session -t name` : kill a session by target name

## See also
- zellij
- screen
- herdr

## Categories
shell

## Links
- More information: https://github.com/tmux/tmux
- tldr page: https://tldr.inbrowser.app/pages/common/tmux
