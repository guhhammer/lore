# git cliff

A highly customizable changelog generator.

## Usage
- `git cliff >CHANGELOG.md` : generate a changelog from all commits in a Git repository and save it to `CHANGELOG.md`
- `git cliff -l` : generate a changelog from commits starting from the latest tag and print it to `stdout`
- `git cliff --current` : generate a changelog from commits that belong to the current tag (use `git checkout` on a tag before this)
- `git cliff -u` : generate a changelog from commits that do not belong to a tag
- `git cliff -i` : write the default config file to `cliff.toml` in the current directory

## Categories
version-control

## Links
- More information: https://git-cliff.org/docs/usage/args/
- tldr page: https://tldr.inbrowser.app/pages/common/git-cliff
- Wikipedia: https://en.wikipedia.org/wiki/Git
