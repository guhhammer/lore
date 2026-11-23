# dnf config-manager

Gestisce le opzioni di configurazione e i repository DNF su sistemi basati su Fedora. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf config-manager --add-repo=repository_url` : aggiunge (e abilita) un repository da un URL
- `dnf config-manager --dump` : stampa i valori di configurazione correnti
- `dnf config-manager --enable repository_id` : abilita un repository specifico
- `dnf config-manager --disable repository_id1 repository_id2 ...` : disabilita i repository specificati
- `dnf config-manager --setopt=option=valore` : imposta un'opzione di configurazione per un repository
- `dnf config-manager --help-cmd` : mostra l'aiuto

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
