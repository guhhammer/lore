# dnf reposync

Sincronizza pacchetti e metadata di un repository DNF remoto in una directory locale. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf reposync --repoid repo_name` : sincronizza tutti i pacchetti dal repository con id `repo_name` in una sottodirectory `repo_name` della directory di lavoro corrente
- `dnf reposync --repoid repo_name -p percorso/della/directory` : sincronizza tutti i pacchetti e definisce una posizione di salvataggio personalizzata
- `dnf reposync --repoid repo_name --download-metadata` : sincronizza tutti i pacchetti e metadati
- `dnf reposync --repoid repo_name -n` : scarica solo i pacchetti più recenti per repository
- `dnf reposync --repoid repo_name -u` : mostra solo gli URL di ciò che verrebbe scaricato, non scaricare
- `dnf reposync --repoid repo_name --remote-time` : tenta di impostare i timestamp dei file scaricati su quelli del lato remoto

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/reposync.html
