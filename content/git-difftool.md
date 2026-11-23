# git difftool

Show file changes using external diff tools. Accepts the same options and arguments as `git diff`.

## Usage
- `git difftool --tool-help` : list available diff tools
- `git config --global diff.tool "meld"` : set the default diff tool to Meld
- `git difftool --staged` : use the default diff tool to show staged changes
- `git difftool -t opendiff commit` : use a specific tool to show changes since a given commit

## See also
- git diff

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-difftool
- tldr page: https://tldr.inbrowser.app/pages/common/git-difftool
