# dagger

Programmable CI/CD engine that runs pipelines in containers locally, in CI, or in the cloud.

## Usage
- `dagger init --sdk go|python|typescript path/to/module` : initialize a new module with the specified SDK
- `dagger develop` : prepare a local module for development
- `dagger functions` : list available functions in the current module
- `dagger call function_name` : call a function on the current module
- `dagger install github.com/user/repo` : install a module dependency from a Git repository
- `dagger update` : update a module's dependencies
- `dagger -h` : display help
- `dagger version` : display version

## Categories
development, system, packages

## Links
- More information: https://docs.dagger.io/reference/cli
- tldr page: https://tldr.inbrowser.app/pages/common/dagger
