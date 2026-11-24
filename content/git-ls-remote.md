# git ls-remote

Git command for listing references in a remote repository based on name or URL. If no name or URL are given, then the configured upstream branch will be used, or remote origin if the former is not configured.

## Usage
- `git ls-remote` : show all references in the default remote repository
- `git ls-remote --heads` : show only heads references in the default remote repository
- `git ls-remote -t` : show only tags references in the default remote repository
- `git ls-remote repository_url` : show all references from a remote repository based on name or URL
- `git ls-remote repository_name "pattern"` : show references from a remote repository filtered by a pattern

## Categories
version-control, network, packages

## Links
- More information: https://git-scm.com/docs/git-ls-remote
- tldr page: https://tldr.inbrowser.app/pages/common/git-ls-remote
- Wikipedia: https://en.wikipedia.org/wiki/Git
