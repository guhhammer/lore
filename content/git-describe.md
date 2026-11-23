# git describe

Give an object a human-readable name based on an available ref.

## Usage
- `git describe` : create a unique name for the current commit (the name contains the most recent annotated tag, the number of additional commits, and the abbreviated commit hash)
- `git describe --abbrev=4` : create a name with 4 digits for the abbreviated commit hash
- `git describe --all` : generate a name with the tag reference path
- `git describe v1.0.0` : describe a Git tag
- `git describe branch_name` : create a name for the last commit of a given branch

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-describe
- tldr page: https://tldr.inbrowser.app/pages/common/git-describe
