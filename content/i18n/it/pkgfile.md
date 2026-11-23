# pkgfile

Cerca file da pacchetti nei repository ufficiali su sistemi basati su Arch.

## Usage
- `sudo pkgfile --update` : sincronizza il database pkgfile
- `pkgfile nome_file` : cerca un pacchetto che possiede un file specifico
- `pkgfile --list pacchetto` : elenca tutti i file forniti da un pacchetto
- `pkgfile --list --binaries pacchetto` : elenca gli eseguibili forniti da un pacchetto
- `pkgfile --ignorecase nome_file` : cerca un pacchetto che possiede un file specifico usando corrispondenza senza distinzione maiuscole/minuscole
- `pkgfile --binaries nome_file` : cerca un pacchetto che possiede un file specifico nelle directory `bin` o `sbin`
- `pkgfile --verbose nome_file` : cerca un pacchetto che possiede un file specifico, mostrando la versione del pacchetto
- `pkgfile --repo nome_repository nome_file` : cerca un pacchetto che possiede un file specifico in un repository specifico

## Links
- More information: https://manned.org/pkgfile
