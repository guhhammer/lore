# pnpm outdated

Check for outdated packages. The check can be limited to a subset of the installed packages by providing arguments (patterns are supported).

## Usage
- `pnpm outdated` : check for outdated packages
- `pnpm outdated -r` : check for outdated dependencies found in every workspace package
- `pnpm outdated --filter package_selector` : filter outdated packages using a package selector
- `pnpm outdated -g` : list outdated packages globally
- `pnpm outdated --long` : print details of outdated packages
- `pnpm outdated --format format` : print outdated dependencies in a specific format
- `pnpm outdated --compatible` : print only versions that satisfy specifications in `package.json`
- `pnpm outdated -D` : check only outdated dev dependencies

## Categories
packages, documents

## Links
- More information: https://pnpm.io/cli/outdated
- tldr page: https://tldr.inbrowser.app/pages/common/pnpm-outdated
