# brew

Homebrew - a package manager for macOS and Linux. Some subcommands such as `install` have their own usage documentation.

## Usage
- `brew install formula|cask` : install the latest stable version of a formula or cask
- `brew list` : list all installed formulae and casks
- `brew upgrade formula|cask` : upgrade an installed formula or cask (if none is given, all installed formulae/casks are upgraded)
- `brew update` : fetch the newest version of Homebrew and of all formulae and casks from the Homebrew source repository
- `brew outdated` : show formulae and casks that have a more recent version available
- `brew search text` : search for available formulae (i.e. packages) and casks (i.e. native macOS `.app` packages)
- `brew info formula|cask` : display information about a formula or a cask (version, installation path, dependencies, etc.)
- `brew doctor` : check the local Homebrew installation for potential problems

## Categories
packages

## Links
- More information: https://docs.brew.sh/Manpage
- tldr page: https://tldr.inbrowser.app/pages/common/brew
