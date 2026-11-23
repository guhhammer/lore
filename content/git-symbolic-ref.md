# git symbolic-ref

Read, change, or delete files that store references.

## Usage
- `git symbolic-ref refs/name ref` : store a reference by a name
- `git symbolic-ref -m "message" refs/name refs/heads/branch_name` : store a reference by name, including a message with a reason for the update
- `git symbolic-ref refs/name` : read a reference by name
- `git symbolic-ref -d refs/name` : delete a reference by name
- `git symbolic-ref -q --short refs/name` : for scripting, hide errors with `--quiet` and use `--short` to simplify ("refs/heads/X" prints as "X")

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-symbolic-ref
- tldr page: https://tldr.inbrowser.app/pages/common/git-symbolic-ref
