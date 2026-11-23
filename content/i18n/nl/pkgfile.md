# pkgfile

Zoek bestanden van pakketten in de officiële repositories op Arch-gebaseerde systemen.

## Usage
- `sudo pkgfile --update` : synchroniseer de pkgfile-database
- `pkgfile bestandsnaam` : zoek naar een pakket dat een specifiek bestand bezit
- `pkgfile --list pakket` : toon alle bestanden die door een pakket worden geleverd
- `pkgfile --list --binaries pakket` : toon uitvoerbare bestanden die door een pakket worden geleverd
- `pkgfile --ignorecase bestandsnaam` : zoek naar een pakket dat een specifiek bestand bezit, met hoofdletterongevoelige matching
- `pkgfile --binaries bestandsnaam` : zoek naar een pakket dat een specifiek bestand bezit in de map `bin` of `sbin`
- `pkgfile --verbose bestandsnaam` : zoek naar een pakket dat een specifiek bestand bezit, met weergave van de pakketversie
- `pkgfile --repo repository_naam bestandsnaam` : zoek naar een pakket dat een specifiek bestand bezit in een specifieke repository

## Links
- More information: https://manned.org/pkgfile
