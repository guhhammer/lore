# emacsclient

Open files in an existing Emacs server.

## Usage
- `emacsclient path/to/file` : open a file in an existing Emacs server (using GUI if available)
- `emacsclient -nw path/to/file` : open a file in console mode (without an X window)
- `emacsclient -c path/to/file` : open a file in a new Emacs window
- `emacsclient -e '(command)'` : evaluate a command, printing the output to `stdout`, and then quit
- `emacsclient -a editor path/to/file` : specify an alternative editor in case no Emacs server is running
- `emacsclient -e '(save-buffers-kill-emacs)'` : stop a running Emacs server and all its instances, asking for confirmation on unsaved files

## See also
- emacs

## Categories
files, editors

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/emacsclient
