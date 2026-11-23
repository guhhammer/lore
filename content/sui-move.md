# sui move

Work with Move source code.

## Usage
- `sui move new project_name` : create a new Move project in the given folder
- `sui move test` : test the Move project in the current directory
- `sui move test --coverage; sui move coverage summary` : test with coverage and get a summary
- `sui move coverage source --module module_name` : find which parts of your code are covered from tests (i.e. explain coverage results)
- `sui move build` : build the Move project in the current directory
- `sui move build --path path/to/package` : build the Move project from the given path
- `sui move migrate path/to/package` : migrate to Move 2024 for the package at the provided path

## Categories
files, development, packages

## Links
- More information: https://docs.sui.io/references/cli/move
- tldr page: https://tldr.inbrowser.app/pages/common/sui-move
