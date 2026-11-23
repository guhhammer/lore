# apptainer registry

Gestisce l'autenticazione ai registri OCI/Docker.

## Usage
- `apptainer registry list` : elenca tutte le credenziali del registro configurate
- `apptainer registry login -u nome_utente docker://registro` : accedi a un registro con un nome utente (verrà richiesta la password)
- `apptainer registry login -u nome_utente oras://registro` : accedi a un registro OCI personalizzato
- `apptainer registry login -u nome_utente -p password docker://registro` : accedi con nome utente e password
- `echo "password" | apptainer registry login -u nome_utente --password-stdin docker://registro` : accedi con una password da `stdin`
- `apptainer registry login --authfile path/to/auth.json -u nome_utente docker://registro` : accedi utilizzando un file di autenticazione personalizzato
- `apptainer registry logout docker://registro` : esci da un registro
- `apptainer registry logout --authfile path/to/auth.json docker://registro` : esci utilizzando un file di autenticazione personalizzato

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_registry.html
