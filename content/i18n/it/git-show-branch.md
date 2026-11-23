# git show-branch

Mostra rami e relativi commit.

## Usage
- `git show-branch nome_ramo|riferimento|commit` : mostra un sommario degli ultimi commit in un ramo
- `git show-branch nome_ramo|riferimento|commit` : confronta commit nella cronologia di più commit o rami
- `git show-branch --remotes` : confronta tutti i rami remoti tracciati
- `git show-branch --all` : confronta i rami locali e remoti
- `git show-branch --all --list` : mostra gli ultimi commit di tutti i rami
- `git show-branch --current commit|nome_ramo|riferimento` : confronta un dato ramo con quello corrente
- `git show-branch --sha1-name --current current|nome_ramo|riferimento` : mostra il nome del commit e non il nome relativo
- `git show-branch --more 5 commit|nome_ramo|riferimento commit|nome_ramo|riferimento ...` : mostra un numero aggiuntivo di commit oltre il predecessore comune

## Links
- More information: https://git-scm.com/docs/git-show-branch
