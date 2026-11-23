# herdr

Terminal workspace manager for AI coding agents. Note: The layout is Session > Workspace > Tab > Pane.

## Usage
- `herdr` : start a new session or attach to the default one
- `herdr status` : show local client and server status
- `herdr --default-config` : generate default config and print to `stdout`
- `<Ctrl b><q>` : detach from the current session (inside a herdr session)
- `<Ctrl b><?>` : show keybinds (inside a herdr session)
- `<Ctrl b><N>` : create new workspace (inside a herdr session)
- `<Ctrl b><c>` : create new tab (inside a herdr session)
- `<Ctrl b><v>|<->` : split pane vertically/horizontally (inside a herdr session)

## See also
- tmux
- zellij
- screen

## Categories
shell

## Links
- More information: https://herdr.dev/docs/cli-reference/
- tldr page: https://tldr.inbrowser.app/pages/common/herdr
