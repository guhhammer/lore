# git fame

Pretty-print Git repository contributions.

## Usage
- `git fame` : calculate contributions for the current Git repository
- `git fame --excl "regex"` : exclude files/directories that match the specified `regex`
- `git fame --since "3 weeks ago|2021-05-13"` : calculate contributions made after the specified date
- `git fame --format pipe|yaml|json|csv|tsv` : display contributions in the specified format
- `git fame -t` : display contributions per file extension
- `git fame -w` : ignore whitespace changes
- `git fame -C` : detect inter-file line moves and copies
- `git fame -M` : detect intra-file line moves and copies

## Categories
version-control, text, files

## Links
- More information: https://manned.org/git-fame
- tldr page: https://tldr.inbrowser.app/pages/common/git-fame
- Wikipedia: https://en.wikipedia.org/wiki/Git
