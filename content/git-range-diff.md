# git range-diff

Compare two commit ranges (e.g. two versions of a branch).

## Usage
- `git range-diff commit_1^! commit_2^!` : diff the changes of two individual commits
- `git range-diff theirs...ours` : diff the changes of ours and theirs from their common ancestor, e.g. after an interactive rebase
- `git range-diff base1..rev1 base2..rev2` : diff the changes of two commit ranges, e.g. to check whether conflicts have been resolved appropriately when rebasing commits from `base1` to `base2`

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-range-diff
- tldr page: https://tldr.inbrowser.app/pages/common/git-range-diff
- Wikipedia: https://en.wikipedia.org/wiki/Git
