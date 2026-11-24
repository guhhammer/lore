# git fast-export

Export the contents and history of a Git repository in a streamable, plain-text format.

## Usage
- `git fast-export --all` : export the entire Git repository history to `stdout`
- `git fast-export --all >path/to/file` : export the entire repository to a file
- `git fast-export main` : export a specific branch only
- `git fast-export --progress n --all >path/to/file` : export with `progress` statements every `n` objects (for showing progress during `git fast-import`)
- `git fast-export --all -- path/to/directory >path/to/file` : export only a specific subdirectory's history

## Categories
version-control, files, packages

## Links
- More information: https://manned.org/git-fast-export
- tldr page: https://tldr.inbrowser.app/pages/common/git-fast-export
- Wikipedia: https://en.wikipedia.org/wiki/Git
