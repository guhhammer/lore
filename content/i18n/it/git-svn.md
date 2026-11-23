# git svn

Operazioni bidirezionali tra repository Subversion e Git.

## Usage
- `git svn clone https://example.com/repo_subversion directory_locale` : clona un repository SVN
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo directory_locale` : clona un repository SVN a partire da uno specifico numero di revisione
- `git svn rebase` : aggiorna una copia locale da un repository SVN remoto
- `git svn fetch` : scarica aggiornamenti da un repository SVN remoto senza spostare l'HEAD Git
- `git svn commit` : invia un commit a un repository SVN

## Links
- More information: https://git-scm.com/docs/git-svn
