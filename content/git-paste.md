# git paste

Send commits to a pastebin site using `pastebinit`. Part of `git-extras`.

## Usage
- `git paste` : send the patches between the current branch and its upstream to a pastebin using `pastebinit`
- `git paste @^` : pass options to `git format-patch` in order to select a different set of commits (`@^` selects the parent of `HEAD`, and so the currently checked out commit is sent)

## Categories
version-control

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-paste
- tldr page: https://tldr.inbrowser.app/pages/common/git-paste
- Wikipedia: https://en.wikipedia.org/wiki/Git
