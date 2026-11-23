# hg push

Push changes from the local repository to a specified destination.

## Usage
- `hg push` : push changes to the "default" remote path
- `hg push path/to/destination_repository` : push changes to a specified remote repository
- `hg push --new-branch` : push a new branch if it does not exist (disabled by default)
- `hg push -r revision` : specify a specific revision changeset to push
- `hg push -b branch` : specify a specific branch to push
- `hg push -B bookmark` : specify a specific bookmark to push

## Categories
version-control, packages

## Links
- More information: https://www.mercurial-scm.org/help/commands/push
- tldr page: https://tldr.inbrowser.app/pages/common/hg-push
