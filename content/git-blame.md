# git blame

Show what commit and author last modified each line of a file.

## Usage
- `git blame path/to/file` : print a file with authorship info (author name and commit hash)
- `git blame -e path/to/file` : print the author's email instead of their name
- `git blame commit path/to/file` : print a file with authorship info as of a specific commit
- `git blame commit~ path/to/file` : print a file with authorship info before a specific commit
- `git blame -L 123 path/to/file` : print a file with authorship info starting at a given line
- `git blame -L start_line,end_line path/to/file` : annotate a specific line range of a file
- `git blame -L '/text/',+10 path/to/file` : annotate 10 lines of a file starting at the first line matching a given text
- `git blame -w -C -C -C path/to/file` : annotate a file ignoring whitespaces and line moves

## Categories
files, version-control, text

## Links
- More information: https://git-scm.com/docs/git-blame
- tldr page: https://tldr.inbrowser.app/pages/common/git-blame
