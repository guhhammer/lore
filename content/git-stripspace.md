# git stripspace

Read text (e.g. commit messages, notes, tags, and branch descriptions) from `stdin` and clean it into the manner used by Git.

## Usage
- `cat path/to/file | git stripspace` : trim whitespace from a file
- `cat path/to/file | git stripspace -s` : trim whitespace and Git comments from a file
- `git <path/to/file stripspace -c` : convert all lines in a file into Git comments

## Categories
version-control, files, text

## Links
- More information: https://git-scm.com/docs/git-stripspace
- tldr page: https://tldr.inbrowser.app/pages/common/git-stripspace
- Wikipedia: https://en.wikipedia.org/wiki/Git
