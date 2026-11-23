# git svn

Opération bidirectionnelle entre un référentiel Subversion et Git.

## Usage
- `git svn clone https://example.com/subversion_repo local_dir` : clone un dépôt SVN
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo local_dir` : clone un dépôt SVN à partir d'une révision donnée
- `git svn rebase` : met à jour le clone local à partir du dépôt SVN distant
- `git svn fetch` : cherche les changements distants dans le dépôt SVN sans les appliquer sur le HEAD
- `git svn commit` : valide sur le SVN

## Links
- More information: https://git-scm.com/docs/git-svn
