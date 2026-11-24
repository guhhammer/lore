# ostree

Version control for binary files similar to `git` but optimized for operating system root filesystems. Foundation for immutable image-based operating systems such as Fedora Silverblue, Fedora IoT, or Fedora CoreOS.

## Usage
- `ostree init --repo path/to/repo` : initialize a repository of the files in `$PWD` with metadata in `$PWD/path/to/repo`
- `ostree commit --repo path/to/repo --branch branch_name` : create a commit (snapshot) of the files
- `ostree ls --repo path/to/repo commit_id` : show files in commit
- `ostree show --repo path/to/repo commit_id` : show metadata of commit
- `ostree log --repo path/to/repo branch_name` : show list of commits
- `ostree summary --repo path/to/repo --view` : show repo summary
- `ostree refs --repo path/to/repo` : show available refs (branches)

## Categories
version-control, files, containers

## Links
- More information: https://ostreedev.github.io/ostree/
- tldr page: https://tldr.inbrowser.app/pages/linux/ostree
- Wikipedia: https://en.wikipedia.org/wiki/OSTree
