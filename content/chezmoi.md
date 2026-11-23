# chezmoi

Manage dotfiles across multiple diverse machines.

## Usage
- `chezmoi init` : set up `chezmoi`, creating a Git repository in `~/.local/share/chezmoi`
- `chezmoi init repository_url` : set up `chezmoi` from existing dotfiles of a Git repository
- `chezmoi add path/to/dotfile1 path/to/dotfile2 ...` : start tracking one or more dotfiles
- `chezmoi re-add path/to/dotfile1 path/to/dotfile2 ...` : update repository with local changes
- `chezmoi edit path/to/dotfile_or_symlink` : edit the source state of a tracked dotfile
- `chezmoi diff` : see pending changes
- `chezmoi apply` : apply the changes
- `chezmoi update` : pull changes from a remote repository and apply them

## See also
- stow
- tuckr
- vcsh
- homeshick

## Categories
files, packages, version-control

## Links
- More information: https://www.chezmoi.io/reference/
- tldr page: https://tldr.inbrowser.app/pages/common/chezmoi
