# yadm clone

Works just like `git clone`. In addition you can pass extra flags to configure your repository. If there is a bootstrap file in the repository, you will be prompted to execute it.

## Usage
- `yadm clone remote_repository_location` : clone an existing repository
- `yadm clone remote_repository_location --bootstrap` : clone an existing repository, then execute the bootstrap file
- `yadm clone remote_repository_location --no-bootstrap` : clone an existing repository and after cloning, do not execute the bootstrap file
- `yadm clone remote_repository_location --w worktree_file` : change the worktree that yadm will use during cloning
- `yadm clone remote_repository_location -b branch` : change the branch that yadm gets files from
- `yadm clone remote_repository_location -f` : override an existing repository local branch

## See also
- git clone

## Categories
packages, version-control, files

## Links
- More information: https://yadm.io/docs/common_commands
- tldr page: https://tldr.inbrowser.app/pages/common/yadm-clone
