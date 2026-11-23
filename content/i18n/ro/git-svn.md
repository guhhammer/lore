# git svn

Operație bidirecțională între un repository Subversion și Git.

## Usage
- `git svn clone https://example.com/subversion_repo director_local` : clonează un repository SVN
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo director_local` : clonează un repository SVN pornind de la un număr de revizie dat
- `git svn rebase` : actualizează clona locală din repository-ul SVN la distanță
- `git svn fetch` : preia actualizări din repository-ul SVN la distanță fără a schimba `HEAD`-ul Git
- `git svn commit` : confirmă înapoi în repository-ul SVN

## Links
- More information: https://git-scm.com/docs/git-svn
