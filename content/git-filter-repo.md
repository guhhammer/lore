# git filter-repo

A versatile tool for rewriting Git history.

## Usage
- `git filter-repo --replace-text <(echo 'find==>replacement')` : replace a sensitive string in all files
- `git filter-repo --path path/to/folder` : extract a single folder, keeping history
- `git filter-repo --path path/to/folder --invert-paths` : remove a single folder, keeping history
- `git filter-repo --path-rename path/to/folder/:` : move everything from sub-folder one level up

## See also
- bfg

## Categories
files, version-control, shell

## Links
- More information: https://github.com/newren/git-filter-repo
- tldr page: https://tldr.inbrowser.app/pages/common/git-filter-repo
