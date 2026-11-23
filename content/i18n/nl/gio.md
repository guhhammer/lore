# gio

Beheer lokale en virtuele bestanden (GVfs). Onderdeel van GLib dat gebruikt wordt in GNOME-gebaseerde systemen.

## Usage
- `gio open pad/naar/bestand` : open een bestand met de standaardapplicatie (bijv. PDF, afbeelding)
- `gio list pad/naar/map` : toon bestanden in een map
- `gio info pad/naar/bestand` : toon informatie over een bestand
- `gio copy pad/naar/bron pad/naar/bestemming` : kopieer een bestand
- `gio trash pad/naar/bestand` : stuur een bestand naar de prullenbak (omkeerbaar)
- `gio trash --empty` : leeg de prullenbak
- `gio launch pad/naar/bestand.desktop` : start een applicatie vanuit een `.desktop` bestand
- `gio set pad/naar/bestand.desktop metadata::trusted true` : markeer een `.desktop` bestand als vertrouwd, waardoor het uitgevoerd kan worden

## Links
- More information: https://manned.org/gio
