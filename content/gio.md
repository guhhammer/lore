# gio

Handle local and virtual files (GVfs). Part of GLib used in GNOME-based systems.

## Usage
- `gio open path/to/file` : open a file with the default application (e.g. PDF, image)
- `gio list path/to/directory` : list files in a directory
- `gio info path/to/file` : show information about a file
- `gio copy path/to/source path/to/destination` : copy a file
- `gio trash path/to/file` : send a file to the trash (reversible)
- `gio trash --empty` : empty the trash
- `gio launch path/to/file.desktop` : launch an application from a `.desktop` file
- `gio set path/to/file.desktop metadata::trusted true` : mark a `.desktop` file as trusted, allowing it to be executed

## Categories
files, desktop

## Links
- More information: https://manned.org/gio
- tldr page: https://tldr.inbrowser.app/pages/linux/gio
