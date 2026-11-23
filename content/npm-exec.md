# npm exec

Execute binaries from `npm` packages.

## Usage
- `npm x command argument1 argument2 ...` : execute the command from a local or remote `npm` package
- `npm x --package package command` : specify the package explicitly (useful if multiple commands with the same name exist)
- `npm x --no-install command argument1 argument2 ...` : run a command if it exists in the current path or in `node_modules/.bin`
- `npm x --quiet command argument1 argument2 ...` : execute a specific command, suppressing any output from `npm` itself
- `npm x --help` : display help

## Categories
packages

## Links
- More information: https://docs.npmjs.com/cli/npm-exec/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-exec
