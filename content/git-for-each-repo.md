# git for-each-repo

Run a Git command on a list of repositories. Note: This command is experimental and may change.

## Usage
- `git for-each-repo --config maintenance.repo maintenance run` : run maintenance on each of a list of repositories stored in the `maintenance.repo` user configuration variable
- `git for-each-repo --config global_configuration_variable pull` : run `git pull` on each repository listed in a global configuration variable

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-for-each-repo
- tldr page: https://tldr.inbrowser.app/pages/common/git-for-each-repo
- Wikipedia: https://en.wikipedia.org/wiki/Git
