# git p4

Import from and submit to Perforce repositories.

## Usage
- `git p4 clone path/to/p4_depot` : clone a Perforce depot into a new Git repository
- `git p4 sync path/to/p4_depot` : sync changes from Perforce into the current Git repository
- `git p4 rebase` : rebase local commits on top of the latest Perforce changes
- `git p4 submit` : submit Git changes back to Perforce
- `git p4 clone path/to/p4_depot@all` : clone the full Perforce history instead of only the latest changelist

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-p4
- tldr page: https://tldr.inbrowser.app/pages/common/git-p4
