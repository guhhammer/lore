# wmctrl

CLI for X Window Manager.

## Usage
- `wmctrl -l` : list all windows, managed by the window manager
- `wmctrl -a window_title` : switch to the first window whose (partial) title matches
- `wmctrl -R window_title` : move a window to the current workspace, raise it, and give it focus
- `wmctrl -s workspace_number` : switch to a workspace
- `wmctrl -r window_title -b toggle,fullscreen` : select a window and toggle fullscreen
- `wmctrl -r window_title -t workspace_number` : select a window and move it to a workspace

## Categories
desktop

## Links
- More information: https://manned.org/wmctrl
- tldr page: https://tldr.inbrowser.app/pages/linux/wmctrl
