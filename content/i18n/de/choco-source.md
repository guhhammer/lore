# choco source

Verwalte die Paketquellen mit Chocolatey.

## Usage
- `choco source list` : gib alle momentan verfügbaren Quellen aus
- `choco source add -n name -s url` : füge eine neue Paketquelle hinzu
- `choco source add -n name -s url -u benutzername -p passwort` : füge eine neue Paketquelle mit Zugangsdaten hinzu
- `choco source add -n name -s url --cert pfad/zu/zertifikat` : füge eine neue Paketquelle mit Client-Zertifikat hinzu
- `choco source enable -n name` : aktiviere eine Paketquelle
- `choco source disable -n name` : deaktiviere eine Paketquelle
- `choco source remove -n name` : entferne eine Paketquelle

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
