# wapm

The WebAssembly package manager.

## Usage
- `wapm init` : interactively create a new `wapm.toml` file
- `wapm install` : download all the packages listed as dependencies in `wapm.toml`
- `wapm install package@version` : download a specific version of a package and add it to the list of dependencies in `wapm.toml`
- `wapm install --global package` : download a package and install it globally
- `wapm uninstall package` : uninstall a package and remove it from the list of dependencies in `wapm.toml`
- `wapm list` : print a tree of locally installed dependencies
- `wapm list --global` : list top-level globally installed packages
- `wapm run command_name arguments` : execute a package command using the Wasmer runtime

## Categories
packages, network

## Links
- More information: https://wasmer.io/help/reference
- tldr page: https://tldr.inbrowser.app/pages/common/wapm
