# lerna

Manage JavaScript projects with multiple packages.

## Usage
- `lerna init` : initialize project files (`lerna.json`, `package.json`, `.git`, etc.)
- `lerna bootstrap` : install all external dependencies of each package and symlink together local dependencies
- `lerna run script` : run a specific script for every package that contains it in its `package.json`
- `lerna exec -- ls` : execute an arbitrary shell command in every package
- `lerna publish` : publish all packages that have changed since the last release

## Categories
packages, development, text

## Links
- More information: https://lerna.js.org/docs/api-reference/commands
- tldr page: https://tldr.inbrowser.app/pages/common/lerna
