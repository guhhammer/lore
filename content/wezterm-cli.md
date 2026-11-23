# wezterm cli

Interact with a running Wezterm GUI or multiplexer.

## Usage
- `wezterm cli list` : list windows, tabs, and panes
- `wezterm cli split-pane --left|right|top|bottom --cells|percent n` : split the current pane and print the new pane's ID to `stdout`
- `wezterm cli activate-pane --pane-id pane_id` : activate (focus) a pane
- `wezterm cli kill-pane --pane-id pane_id` : kill a pane

## Links
- More information: https://wezterm.org/cli/cli/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/wezterm-cli
