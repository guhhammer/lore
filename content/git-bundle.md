# git bundle

Package objects and references into an archive.

## Usage
- `git bundle create path/to/file.bundle branch_name` : create a bundle file that contains all objects and references of a specific branch
- `git bundle create path/to/file.bundle --all` : create a bundle file of all branches
- `git bundle create path/to/file.bundle -5 HEAD` : create a bundle file of the last 5 commits of the current branch
- `git bundle create path/to/file.bundle --since 7.days HEAD` : create a bundle file of the latest 7 days
- `git bundle verify path/to/file.bundle` : verify that a bundle file is valid and can be applied to the current repository
- `git bundle unbundle path/to/file.bundle` : print to `stdout` the list of references contained in a bundle
- `git pull path/to/file.bundle branch_name` : unbundle a specific branch from a bundle file into the current repository
- `git clone path/to/file.bundle` : create a new repository from a bundle

## Categories
files, version-control, packages

## Links
- More information: https://git-scm.com/docs/git-bundle
- tldr page: https://tldr.inbrowser.app/pages/common/git-bundle
- Wikipedia: https://en.wikipedia.org/wiki/Git
