# git merge-index

Run a merge program on files that need merging.

## Usage
- `git merge-index git-merge-one-file -a` : merge all files needing resolution using the standard helper
- `git merge-index git-merge-one-file -- path/to/file` : merge a specific file
- `git merge-index -o git-merge-one-file -- path/to/file1 path/to/file2 ...` : merge multiple files, continuing on failures
- `git merge-index -q merge_program -a` : quietly merge all files with a custom program
- `git merge-index cat -- path/to/file` : inspect merge inputs for a file using `cat`

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-merge-index
- tldr page: https://tldr.inbrowser.app/pages/common/git-merge-index
