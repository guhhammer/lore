# apptainer config

Gestisce varie configurazioni di Apptainer.

## Usage
- `sudo apptainer config fakeroot -a nome_utente` : aggiunge un mapping utente fakeroot
- `sudo apptainer config fakeroot -r nome_utente` : rimuove un mapping utente fakeroot
- `sudo apptainer config fakeroot -e nome_utente` : abilita un mapping utente fakeroot
- `sudo apptainer config fakeroot -d nome_utente` : disabilita un mapping utente fakeroot
- `sudo apptainer config global -g direttiva` : ottiene il valore corrente di una direttiva di configurazione globale
- `sudo apptainer config global -s direttiva valore` : imposta il valore di una direttiva di configurazione globale
- `sudo apptainer config global -u direttiva valore` : annulla il valore di una direttiva di configurazione globale
- `sudo apptainer config global -r direttiva` : reimposta una direttiva di configurazione globale al suo valore predefinito

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_config.html
