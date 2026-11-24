# git grep

Find strings inside tracked files in a repository. Accepts a lot of the same flags as regular `grep`.

## Usage
- `git grep "search_string"` : search for a string in files in the current `HEAD`
- `git grep "search_string" -- "*.ext"` : search for a string in files matching a `glob` pattern in the current `HEAD`
- `git grep --recurse-submodules "search_string"` : search for a string, including submodules
- `git grep "search_string" HEAD~2` : search for a string at a specific point in history
- `git grep "search_string" $(git rev-list --all)` : search for a string across all branches and all of history

## Categories
version-control, text, packages

## Links
- More information: https://git-scm.com/docs/git-grep
- tldr page: https://tldr.inbrowser.app/pages/common/git-grep
- Wikipedia: https://en.wikipedia.org/wiki/Git
