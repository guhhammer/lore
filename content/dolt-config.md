# dolt config

Read and write local (per repository) and global (per user) Dolt configuration variables.

## Usage
- `dolt config --list` : list all local and global configuration options and their values
- `dolt config --get name` : display the value of a local or global configuration variable
- `dolt config --add name value` : modify the value of a local configuration variable, creating it if it doesn't exist
- `dolt config --global --add name value` : modify the value of a global configuration variable, creating it if it doesn't exist
- `dolt config --unset name` : delete a local configuration variable
- `dolt config --global --unset name` : delete a global configuration variable

## Categories
packages, users

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-config
- tldr page: https://tldr.inbrowser.app/pages/common/dolt-config
