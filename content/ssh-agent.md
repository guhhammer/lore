# ssh-agent

Spawn an SSH Agent process. Note: An SSH Agent holds SSH keys decrypted in memory until removed or the process is killed.

## Usage
- `eval $(ssh-agent)` : start an SSH Agent for the current shell
- `ssh-agent -k` : kill the currently running agent

## See also
- ssh-add

## Categories
network, processes, monitoring

## Links
- More information: https://man.openbsd.org/ssh-agent
- tldr page: https://tldr.inbrowser.app/pages/common/ssh-agent
- Wikipedia: https://en.wikipedia.org/wiki/Ssh-agent
