# git secret

Store private data inside a Git repository. Written in Bash.

## Usage
- `git secret init` : initialize `git-secret` in a local repository
- `git secret tell -m` : grant access to the current Git user's email
- `git secret tell email` : grant access by email
- `git secret killperson email` : revoke access by email
- `git secret whoknows` : list emails with access to secrets
- `git secret add path/to/file` : register a secret file
- `git secret hide` : encrypt secrets
- `git secret reveal` : decrypt secret files

## Categories
version-control, packages, files

## Links
- More information: https://github.com/sobolevn/git-secret
- tldr page: https://tldr.inbrowser.app/pages/common/git-secret
