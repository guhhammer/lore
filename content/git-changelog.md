# git changelog

Generate a changelog report from repository commits and tags. Part of `git-extras`.

## Usage
- `git changelog` : update existing file or create a new `History.md` file with the commit messages since the latest Git tag
- `git changelog -l` : list commits from the current version
- `git changelog -l -s 2.1.0` : list a range of commits from the tag named `2.1.0` to now
- `git changelog -s 0.5.0 -f 1.0.0` : list pretty formatted range of commits between the tag `0.5.0` and the tag `1.0.0`
- `git changelog --start-commit 0b97430 -f 1.0.0` : list pretty formatted range of commits between the commit `0b97430` and the tag `1.0.0`
- `git changelog CHANGELOG.md` : specify `CHANGELOG.md` as the output file
- `git changelog -p` : replace contents of current changelog file entirely

## Categories
version-control, files, packages

## Links
- More information: https://manned.org/git-changelog
- tldr page: https://tldr.inbrowser.app/pages/common/git-changelog
