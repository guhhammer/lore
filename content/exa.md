# exa

A modern replacement for `ls` (List directory contents). Note: `exa` is no longer maintained. Use `eza` instead.

## Usage
- `exa -1` : list files one per line
- `exa -a` : list all files, including hidden files
- `exa -l -a` : list all files in long format (permissions, ownership, size, and modification date)
- `exa -r -s size` : list files with the largest at the top
- `exa -l -T -L 3` : display a tree of files, three levels deep
- `exa -l -s modified` : list files sorted by modification date (oldest first)
- `exa -l -h --icons --git` : list files with their headers, icons, and Git statuses
- `exa --git-ignore` : don't list files mentioned in `.gitignore`

## See also
- eza

## Categories
files, version-control

## Links
- More information: https://github.com/ogham/exa#command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/exa
