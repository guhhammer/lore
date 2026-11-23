# eselect repository

Un modulo `eselect` per la configurazione dei repository ebuild per Portage. Dopo aver abilitato un repository, è necessario eseguire `emerge --sync nome_repo` per scaricare gli ebuild.

## Usage
- `eselect repository list` : elenca tutti i repository ebuild registrati su <https://repos.gentoo.org>
- `eselect repository list -i` : elenca i repository abilitati
- `eselect repository enable nome|indice` : abilita un repository dall'elenco per nome o indice dal comando `list`
- `eselect repository add nome rsync|git|mercurial|svn|... sync_uri` : abilita un repository non registrato
- `eselect repository disable repo1 repo2 ...` : disabilita i repository senza rimuovere il loro contenuto
- `eselect repository remove repo1 repo2 ...` : disabilita i repository e rimuove il loro contenuto
- `eselect repository create nome path/to/repo` : crea un repository locale e lo abilita

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect/Repository
