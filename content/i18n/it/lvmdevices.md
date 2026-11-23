# lvmdevices

Gestisce il file dei dispositivi LVM che elenca i dispositivi di blocco consentiti per i volumi fisici.

## Usage
- `sudo lvmdevices` : elenca i dispositivi registrati nel file dei dispositivi
- `sudo lvmdevices --adddev /dev/sdXN` : aggiunge un dispositivo al file dei dispositivi
- `sudo lvmdevices --deldev /dev/sdXN` : rimuove un dispositivo dal file dei dispositivi
- `sudo lvmdevices --addpvid pvid` : aggiunge un volume fisico tramite il suo PVID
- `sudo lvmdevices --delpvid pvid` : rimuove un volume fisico tramite il suo PVID
- `sudo lvmdevices --update` : aggiorna il file dei dispositivi dopo il cambio di nomi dei dispositivi
- `sudo lvmdevices --check` : controlla il file dei dispositivi per problemi
- `lvmdevices --version` : mostra la versione

## Links
- More information: https://manned.org/lvmdevices
