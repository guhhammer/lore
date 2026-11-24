# git feature

Create or merge feature branches. Feature branches obey the format feature/name.

## Usage
- `git feature feature_branch` : create and switch to a new feature branch
- `git feature finish feature_branch` : merge a feature branch into the current branch creating a merge commit
- `git feature finish --squash feature_branch` : merge a feature branch into the current branch squashing the changes into one commit
- `git feature feature_branch -r remote_name` : send changes from a specific feature branch to its remote counterpart

## Categories
version-control

## Links
- More information: https://manned.org/git-feature
- tldr page: https://tldr.inbrowser.app/pages/common/git-feature
- Wikipedia: https://en.wikipedia.org/wiki/Git
