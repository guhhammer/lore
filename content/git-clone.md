# git clone

Clone an existing repository.

## Usage
- `git clone remote_repository_location path/to/directory` : clone an existing repository into a new directory (the default directory is the repository name)
- `git clone --recursive remote_repository_location` : clone an existing repository and its submodules
- `git clone -n remote_repository_location` : clone only the `.git` directory of an existing repository
- `git clone -l path/to/local_repository` : clone a local repository
- `git clone -q remote_repository_location` : clone quietly
- `git clone --depth 10 remote_repository_location` : clone an existing repository only fetching the 10 most recent commits on the default branch (useful to save time)
- `git clone -b name --single-branch remote_repository_location` : clone an existing repository only fetching a specific branch
- `git clone -c core.sshCommand="ssh -i path/to/private_ssh_key" remote_repository_location` : clone an existing repository using a specific SSH command

## Categories
version-control, packages, files

## Links
- More information: https://git-scm.com/docs/git-clone
- tldr page: https://tldr.inbrowser.app/pages/common/git-clone
- Wikipedia: https://en.wikipedia.org/wiki/Git
