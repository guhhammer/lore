# dep

Deploy PHP applications. Note: The Go command `dep` with the same name is deprecated and archived.

## Usage
- `dep init` : interactively initialize deployer in the local path (use a framework template with `--template=template`)
- `dep deploy hostname` : deploy an application to a remote host
- `dep rollback` : roll back to the previous working release
- `dep ssh hostname` : connect to a remote host via SSH
- `dep list` : list commands
- `dep run "command"` : run any arbitrary command on the remote hosts
- `dep help command` : display help for a command

## Categories
network

## Links
- More information: https://deployer.org/docs/8.x/cli
- tldr page: https://tldr.inbrowser.app/pages/common/dep
