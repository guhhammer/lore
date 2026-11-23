# jj config

Manage config options. Some subcommands such as `edit`, `get`, `list`, `path`, `set`, and `unset` have their own usage documentation.

## Usage
- `jj config e --user` : start an editor on the user-level config file
- `jj config g name` : get the value of a config option
- `jj config l` : list all config variables and their values
- `jj config l name` : list the value of a specific config option
- `jj config p --user` : print the path to the user-level config file
- `jj config s --user name value` : set a config option in the user-level config
- `jj config u --user name` : unset a config option in the user-level config
- `jj config gc` : find and delete stale repo-level config directories

## Categories
users, files, documents

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-config
- tldr page: https://tldr.inbrowser.app/pages/common/jj-config
