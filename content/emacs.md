# emacs

The extensible, customizable, self-documenting, real-time display editor.

## Usage
- `emacs path/to/file` : start Emacs and open a file
- `emacs +line_number path/to/file` : open a file at a specified line number
- `emacs --script path/to/file.el` : run an Emacs Lisp file as a script
- `emacs -nw` : start Emacs in console mode (without an X window)
- `emacs --daemon` : start an Emacs server in the background (accessible via `emacsclient`)
- `emacsclient --eval '(save-buffers-kill-emacs)'` : stop a running Emacs server and all its instances, asking for confirmation on unsaved files
- `<Ctrl x><Ctrl s>` : save a file in Emacs
- `<Ctrl x><Ctrl c>` : quit Emacs

## See also
- emacsclient

## Categories
editors

## Links
- More information: https://www.gnu.org/software/emacs
- tldr page: https://tldr.inbrowser.app/pages/common/emacs
- Wikipedia: https://en.wikipedia.org/wiki/Emacs
