# git imerge

Perform a merge or rebase between two Git branches incrementally. Conflicts between branches are tracked down to pairs of individual commits, to simplify conflict resolution.

## Usage
- `git imerge rebase branch_to_rebase_onto` : start imerge-based rebase (check out the branch to be rebased, first)
- `git imerge merge branch_to_be_merged` : start imerge-based merge (check out the branch to merge into, first)
- `git imerge diagram` : show ASCII diagram of in-progress merge or rebase
- `git imerge continue --no-edit` : continue imerge operation after resolving conflicts (`git add` the conflicted files, first)
- `git imerge finish` : wrap up imerge operation, after all conflicts are resolved
- `git imerge remove && git checkout previous_branch` : abort imerge operation, and return to the previous branch

## Categories
version-control

## Links
- More information: https://github.com/mhagger/git-imerge
- tldr page: https://tldr.inbrowser.app/pages/common/git-imerge
