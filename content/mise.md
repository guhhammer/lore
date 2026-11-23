# mise

Manage language runtimes like Node.js, Python, Ruby, Go, Java, etc. and various tools.

## Usage
- `mise plugins list-all` : list all available plugins
- `mise plugins add name` : install a plugin
- `mise ls-remote name` : list runtime versions available for install
- `mise install name@version` : install a specific version of a package
- `mise use -g name@version` : set global version for a package
- `mise use name@version` : set local version for a package
- `mise set variable=value` : set environment variable in configuration
- `mise use name\[option1=option1_value,option2=option2_value\]@version` : pass plugin options

## Categories
packages

## Links
- More information: https://mise.jdx.dev/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/mise
