# git push

Push commits to a remote repository.

## Usage
- `git push` : send local changes in the current branch to its default remote counterpart
- `git push remote_name local_branch` : send changes from a specific local branch to its remote counterpart
- `git push -u remote_name local_branch` : send changes from a specific local branch to its remote counterpart, and set the remote one as the default push/pull target of the local one
- `git push remote_name local_branch:remote_branch` : send changes from a specific local branch to a specific remote branch
- `git push --all remote_name` : send changes on all local branches to their counterparts in a given remote repository
- `git push remote_name -d remote_branch` : delete a branch in a remote repository
- `git push --prune remote_name` : remove remote branches that don't have a local counterpart
- `git push --tags` : publish tags that aren't yet in the remote repository

## Categories
version-control, packages

## Links
- More information: https://git-scm.com/docs/git-push
- tldr page: https://tldr.inbrowser.app/pages/common/git-push
- Wikipedia: https://en.wikipedia.org/wiki/Git
