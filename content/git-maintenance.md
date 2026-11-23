# git maintenance

Run tasks to optimize Git repository data.

## Usage
- `git maintenance register` : register the current repository in the user's list of repositories to daily have maintenance run
- `git maintenance start` : schedule maintenance tasks to run on the current repository every hour
- `git maintenance stop` : halt the background maintenance schedule for the current repository
- `git maintenance unregister` : remove the current repository from the user's maintenance repository list
- `git maintenance run --task commit-graph|gc|incremental-repack|loose-objects|pack-refs|prefetch` : run a specific maintenance task on the current repository

## Categories
version-control, packages

## Links
- More information: https://git-scm.com/docs/git-maintenance
- tldr page: https://tldr.inbrowser.app/pages/common/git-maintenance
