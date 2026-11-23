# dnf changelog

Visualizza i changelog per un dato pacchetto. Non predefinito in `dnf` ma supportato tramite `dnf-plugins-core`.

## Usage
- `dnf changelog pacchetto` : visualizza tutti i changelog per un dato pacchetto
- `dnf changelog --since date pacchetto` : visualizza tutti i changelog per un dato pacchetto dopo una data specificata
- `dnf changelog --count number pacchetto` : visualizza gli ultimi `n` changelog per un dato pacchetto
- `dnf changelog --upgrades pacchetto` : mostra solo i nuovi elementi per i pacchetti aggiornabili
- `dnf changelog --help-cmd` : mostra l'aiuto

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/changelog.html
