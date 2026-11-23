# dnf changelog

View the changelogs for a given package. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf changelog package` : view all changelogs for a given package
- `dnf changelog --since date package` : view all changelogs for a given package after a specified date
- `dnf changelog --count number package` : view the last `n` number of changelogs for a given package
- `dnf changelog --upgrades package` : show only new items for upgradeable packages
- `dnf changelog --help-cmd` : display help

## Categories
packages

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/changelog.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-changelog
