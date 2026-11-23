# rg

Ripgrep, a recursive line-oriented search tool. Aims to be a faster alternative to `grep`.

## Usage
- `rg pattern` : recursively search current directory for a pattern (`regex`)
- `rg pattern path/to/file_or_directory` : recursively search for a pattern in a file or directory
- `rg -F -- string` : search for a literal string pattern
- `rg -. --no-ignore pattern` : include hidden files and entries listed in `.gitignore`
- `rg pattern -g 'filename_glob_pattern'` : only search the files matching the `glob` pattern(s) (e.g. `README.*`, use `!filename_pattern` to exclude instead)
- `rg --files | rg --passthru pattern` : recursively list filenames in the current directory and highlight ones that match a pattern
- `rg -l pattern` : only list matched files (useful when piping to other commands)
- `rg -v pattern` : show lines that do not match the pattern

## Categories
text, files

## Links
- More information: https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md
- tldr page: https://tldr.inbrowser.app/pages/common/rg
