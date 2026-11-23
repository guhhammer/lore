# devenv

Fast, declarative, reproducible, and composable developer environments using Nix.

## Usage
- `devenv init` : initialize the environment
- `devenv shell --impure` : enter the Development Environment with relaxed hermeticity (state of being airtight)
- `devenv info --verbose` : get detailed information about the current environment
- `devenv up --config path/to/file` : start processes with `devenv`
- `devenv --clean --offline` : clean the environment variables and re-enter the shell in offline mode
- `devenv gc` : delete the previous shell generations

## Categories
shell

## Links
- More information: https://devenv.sh/getting-started/#commands
- tldr page: https://tldr.inbrowser.app/pages/common/devenv
