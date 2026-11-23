# etckeeper

Track system configuration files in Git.

## Usage
- `sudo etckeeper init` : set up a Git repo and perform various setup tasks (run from `/etc`)
- `sudo etckeeper commit message` : commit all changes in `/etc`
- `sudo etckeeper vcs status` : run arbitrary Git commands
- `sudo etckeeper unclean` : check if there are uncommitted changes (only returns an exit code)
- `sudo etckeeper uninit` : destroy existing repo and stop tracking changes

## Categories
version-control, users

## Links
- More information: https://manned.org/etckeeper
- tldr page: https://tldr.inbrowser.app/pages/linux/etckeeper
