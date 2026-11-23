# pipx

Installa ed esegui applicazioni Python in ambienti isolati.

## Usage
- `pipx run pycowsay moo` : esegui un'applicazione in un ambiente virtuale temporaneo
- `pipx install pacchetto` : installa un pacchetto in un ambiente virtuale e aggiungi i suoi punti di ingresso al path
- `pipx uninstall pacchetto` : disinstalla un pacchetto
- `pipx list` : elenca i pacchetti installati
- `pipx run --spec httpx-cli httpx http://www.github.com` : esegui un'applicazione in un ambiente virtuale temporaneo con un nome di pacchetto diverso dall'eseguibile
- `pipx inject pacchetto dipendenza1 dipendenza2 ...` : inietta dipendenze in un ambiente virtuale esistente
- `pipx install --pip-args='argomenti_pip' pacchetto` : installa un pacchetto in un ambiente virtuale con argomenti pip
- `pipx upgrade-all|uninstall-all|reinstall-all` : aggiorna, reinstalla o disinstalla tutti i pacchetti installati

## Links
- More information: https://manned.org/pipx
