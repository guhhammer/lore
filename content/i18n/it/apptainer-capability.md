# apptainer capability

Gestisce le capacità Linux per utenti e gruppi.

## Usage
- `apptainer capability avail` : mostra tutte le capacità Linux disponibili
- `apptainer capability avail cap_chown,cap_net_raw,...` : mostra le descrizioni per capacità specifiche
- `apptainer capability list` : elenca le capacità per tutti gli utenti e gruppi
- `apptainer capability list nome_utente|nome_gruppo` : elenca le capacità per un utente o gruppo specifico
- `sudo apptainer capability add -u nome_utente cap_net_raw,cap_chown,...` : aggiunge capacità a un utente
- `sudo apptainer capability add -g nome_gruppo cap_net_raw,cap_chown,...` : aggiunge capacità a un gruppo
- `sudo apptainer capability drop -u nome_utente cap_net_raw,cap_chown,...` : rimuove capacità da un utente
- `sudo apptainer capability drop -u nome_utente all` : rimuove tutte le capacità da un utente

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_capability.html
