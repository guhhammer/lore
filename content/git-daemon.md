# git daemon

A really simple server for Git repositories.

## Usage
- `git daemon --export-all path/to/directory1 path/to/directory2 ...` : launch a Git daemon with a whitelisted set of directories
- `git daemon --base-path=path/to/directory --export-all --reuseaddr` : launch a Git daemon with a specific base directory and allow pulling from all sub-directories that look like Git repositories
- `git daemon path/to/directory --enable=receive-pack --informative-errors --verbose` : launch a Git daemon for the specified directory, verbosely printing log messages and allowing Git clients to write to it

## Categories
version-control, files, system

## Links
- More information: https://git-scm.com/docs/git-daemon
- tldr page: https://tldr.inbrowser.app/pages/common/git-daemon
