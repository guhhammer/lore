# stow

Symlink manager. Often used to manage dotfiles.

## Usage
- `stow -t path/to/target_directory path/to/file_or_directory1 path/to/file_or_directory2 ...` : symlink all files recursively to a given directory
- `stow -D -t path/to/target_directory path/to/file_or_directory1 path/to/file_or_directory2 ...` : delete symlinks recursively from a given directory
- `stow -n -t path/to/target_directory path/to/file_or_directory1 path/to/file_or_directory2 ...` : simulate to see what the result would be like
- `stow -R -t path/to/target_directory path/to/file_or_directory1 path/to/file_or_directory2 ...` : delete and resymlink
- `stow --ignore=regex -t path/to/target_directory path/to/file_or_directory1 path/to/file_or_directory2 ...` : exclude files matching a `regex`

## See also
- chezmoi
- tuckr
- vcsh
- homeshick

## Categories
files

## Links
- More information: https://www.gnu.org/software/stow/manual/stow.html#Invoking-Stow
- tldr page: https://tldr.inbrowser.app/pages/common/stow
