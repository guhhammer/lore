# salt

Execute commands and assert state on remote salt minions.

## Usage
- `salt '*' test.ping` : list connected minions
- `salt '*' state.highstate` : execute a highstate on all connected minions
- `salt '*.example.com' pkg.upgrade` : upgrade packages using the OS package manager (apt, yum, brew) on a subset of minions
- `salt 'minion_id' cmd.run "ls "` : execute an arbitrary command on a particular minion

## Categories
packages, network

## Links
- More information: https://docs.saltproject.io/en/latest/ref/cli/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/salt
