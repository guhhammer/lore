# git clean

Remove files not tracked by Git from the working tree.

## Usage
- `git clean -i` : interactively delete untracked files
- `git clean -n` : simulate the cleanup, showing which files would be deleted
- `git clean -f` : immediately force deletion of all untracked files
- `git clean -f -d` : delete untracked directories
- `git clean -f -- path/to/directory '*.ext'` : delete only untracked files matching specific paths or `glob` patterns
- `git clean -f -e '*.ext' -e path/to/directory/` : delete untracked files except those matching the given patterns
- `git clean -f -x` : delete untracked files and excluded files (those listed in `.gitignore` and `.git/info/exclude`)

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-clean
- tldr page: https://tldr.inbrowser.app/pages/common/git-clean
