# eza

Modern, maintained replacement for `ls`, built on `exa`.

## Usage
- `eza -1` : list files one per line
- `eza -a` : list all files, including hidden files
- `eza -al` : list all files in long format (permissions, ownership, size, and modification date)
- `eza -r -s size` : list files with the largest at the top
- `eza -lT -L 3` : display a tree of files, three levels deep
- `eza -l -s modified` : list files sorted by modification date (oldest first)
- `eza -lh --icons --git` : list files with their headers, icons, and Git statuses
- `eza --git-ignore` : don't list files mentioned in `.gitignore`

## Categories
version-control

## Links
- More information: https://github.com/eza-community/eza
- tldr page: https://tldr.inbrowser.app/pages/common/eza
