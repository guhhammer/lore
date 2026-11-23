# git svn

Operación bidreccional entre un repositorio Subversión y otro Git.

## Usage
- `git svn clone https://example.com/repositorio_subversion directorio_local` : clona un repositorio SVN
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo directorio_local` : clona un repositorio SVN a partir de un número de revisión específico
- `git svn rebase` : actualiza el clon local a partir del repositorio SVN
- `git svn fetch` : obtiene las actualizaciones del repositorio SVN remoto sin cambiar el HEAD de Git
- `git svn commit` : realiza una confirmación en un repositorio SVN

## Links
- More information: https://git-scm.com/docs/git-svn
