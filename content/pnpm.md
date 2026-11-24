# pnpm

Fast, disk space efficient package manager for Node.js. Manage Node.js projects and their module dependencies.

## Usage
- `pnpm init` : create a `package.json` file
- `pnpm i` : download all the packages listed as dependencies in `package.json`
- `pnpm add module_name@version` : download a specific version of a package and add it to the list of dependencies in `package.json`
- `pnpm add module_name -D` : download a package and add it to the list of dev dependencies in `package.json`
- `pnpm add module_name -g` : download a package and install it globally
- `pnpm rm module_name` : uninstall a package and remove it from the list of dependencies in `package.json`
- `pnpm ls` : print a tree of locally installed modules
- `pnpm ls -g --depth 0` : list top-level globally installed modules

## Categories
packages, network, text

## Links
- More information: https://pnpm.io/pnpm-cli
- tldr page: https://tldr.inbrowser.app/pages/common/pnpm
- Wikipedia: https://en.wikipedia.org/wiki/Pnpm
