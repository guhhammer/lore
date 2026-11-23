# gitui

A lightweight keyboard-only TUI for Git.

## Usage
- `gitui` : start `gitui` for the repository in the current directory
- `gitui -t theme2.ron` : specify the filename of the color theme loaded from the config directory (defaults to `theme.ron`)
- `gitui --logfile path/to/file` : store logging output into a specific file
- `gitui -f path/to/file` : inspect a specific file inside the repository in the current directory
- `gitui --watcher` : use notify-based filesystem watcher instead of tick-based update
- `gitui --bugreport` : generate a bug report
- `gitui -d path/to/directory` : use a specific Git directory
- `gitui -w path/to/directory` : use a specific working directory

## See also
- tig
- git-gui

## Categories
files, version-control, hardware

## Links
- More information: https://github.com/gitui-org/gitui
- tldr page: https://tldr.inbrowser.app/pages/common/gitui
