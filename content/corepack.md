# corepack

Zero-runtime-dependency package acting as bridge between Node projects and their package managers.

## Usage
- `corepack enable` : add the Corepack shims to the Node.js installation directory to make them available as global commands
- `corepack enable --install-directory path/to/directory` : add the Corepack shims to a specific directory
- `corepack disable` : remove the Corepack shims from the Node.js installation directory
- `corepack prepare package_manager@version --activate` : prepare a specific package manager
- `corepack prepare` : prepare the package manager configured for the project in the current path
- `corepack npm|pnpm|yarn package_manager_arguments` : use a package manager without installing it as a global command
- `corepack hydrate path/to/corepack.tgz` : install a package manager from the specified archive
- `corepack subcommand --help` : display help for a subcommand

## Categories
packages, files

## Links
- More information: https://github.com/nodejs/corepack
- tldr page: https://tldr.inbrowser.app/pages/common/corepack
