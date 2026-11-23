# gtrash

A featureful trash manager, alternative to `rm` and `trash-cli`.

## Usage
- `gtrash put path/to/file_or_directory1 path/to/file_or_directory2 ...` : move files or directories to the trash can
- `gtrash find regex` : list trashed files whose original path matches a `regex`
- `gtrash r` : open an interactive TUI to select and restore trashed files
- `gtrash find regex --rm` : permanently delete trashed files whose original path matches a `regex`
- `gtrash prune --day days` : permanently delete trashed files older than a number of days
- `gtrash summary` : display the location, number of items, and total size of each trash can

## Categories
files, text

## Links
- More information: https://github.com/umlx5h/gtrash#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gtrash
