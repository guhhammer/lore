# jj fix

Update files with formatting fixes or other content transformations. Revisions can be reviewed afterwards with `jj op show -p`.

## Usage
- `jj fix` : fix changed files in all mutable revisions connected to the working-copy commit
- `jj fix -s revsets` : fix changed files in the specified revision(s) and their descendants
- `jj fix -s @` : fix files only in the working-copy commit
- `jj fix path/to/file_or_directory` : fix only a specific file or directory
- `jj fix -a` : fix all lines instead of only modified lines
- `jj fix -a path/to/file` : format all lines in a specific file
- `jj fix --include-unchanged-files` : fix unchanged files in addition to changed ones across the entire repository
- `jj fix --include-unchanged-files path/to/file_or_directory` : fix unchanged files in addition to changed ones for specific paths

## Categories
files, text

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-fix
- tldr page: https://tldr.inbrowser.app/pages/common/jj-fix
