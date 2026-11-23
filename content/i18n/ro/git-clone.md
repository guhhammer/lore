# git clone

Clonează un repository existent.

## Usage
- `git clone locație_repository_la_distanță cale/către/director` : clonează un repository existent într-un director nou (directorul implicit este numele repository-ului)
- `git clone --recursive locație_repository_la_distanță` : clonează un repository existent împreună cu submodulele lui
- `git clone -n locație_repository_la_distanță` : clonează doar directorul `.git` al unui repository existent
- `git clone -l cale/către/repository_local` : clonează un repository local
- `git clone -q locație_repository_la_distanță` : clonează în mod silențios
- `git clone --depth 10 locație_repository_la_distanță` : clonează un repository existent descărcând doar cele 10 commit-uri cele mai recente de pe ramura implicită (util pentru a economisi timp)
- `git clone -b nume --single-branch locație_repository_la_distanță` : clonează un repository existent descărcând doar o ramură specifică
- `git clone -c core.sshCommand="ssh -i cale/către/cheie_ssh_privata" locație_repository_la_distanță` : clonează un repository existent folosind o comandă SSH specifică

## Links
- More information: https://git-scm.com/docs/git-clone
