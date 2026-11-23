# jj config set

Set a config option to a given value. The value is specified as a TOML expression.

## Usage
- `jj config s --user user.name "name"` : set the user name in the user-level config
- `jj config s --user user.email "email"` : set the user email in the user-level config
- `jj config s --repo name value` : set a config option in the repo-level config
- `jj config s --workspace name value` : set a config option in the workspace-level config
- `jj config s --user name true|false` : set a boolean config option

## See also
- jj config unset

## Categories
users

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-config-set
- tldr page: https://tldr.inbrowser.app/pages/common/jj-config-set
