# bun pm

A set of utilities for working with Bun's package manager. Some subcommands such as `pack`, `pkg`, etc. have their own usage documentation.

## Usage
- `bun pm pack` : create a tarball of the current workspace
- `bun pm bin` : print the path to the `bin` directory
- `bun pm default-trusted` : display the default trusted dependencies
- `bun pm whoami` : print the npm registry username
- `bun pm hash` : generate and print the hash of the current lockfile
- `bun pm untrusted` : list the current untrusted dependencies that contain scripts
- `bun pm migrate` : migrate another package manager's lockfile without installing anything
- `bun pm pkg get property` : get a property from `package.json`

## Categories
packages, documents, monitoring

## Links
- More information: https://bun.com/docs/pm/cli/pm
- tldr page: https://tldr.inbrowser.app/pages/common/bun-pm
