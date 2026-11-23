# bunx

Execute a package binary (installed locally or fetched remotely). Note: `bun x` can be used as an alias for `bunx`.

## Usage
- `bunx package_name "command_argument"` : download and execute a package from the registry
- `bunx package_name --version` : check the version of a locally installed package (if found)
- `bunx --bun package_name` : force an executable to run with the Bun runtime (instead of Node)
- `bunx -p package_name command` : execute a binary that has a different name than its package
- `bunx package_name@version "command_argument"` : download and execute a specific version of a package

## Categories
packages, network, shell

## Links
- More information: https://bun.com/docs/pm/bunx
- tldr page: https://tldr.inbrowser.app/pages/common/bunx
