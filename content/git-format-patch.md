# git format-patch

Prepare `.patch` files. Useful when emailing commits elsewhere.

## Usage
- `git format-patch origin` : create an auto-named `.patch` file for all the unpushed commits
- `git format-patch revision_1..revision_2` : write a `.patch` file for all the commits between 2 revisions to `stdout`
- `git format-patch -n` : write a `.patch` file for the `n` latest commits

## See also
- git am

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-format-patch
- tldr page: https://tldr.inbrowser.app/pages/common/git-format-patch
- Wikipedia: https://en.wikipedia.org/wiki/Git
