# chroot

Voer een commando of interactieve shell uit met een speciale hoofdmap.

## Usage
- `sudo chroot pad/naar/nieuwe_hoofdmap` : voer `$SHELL` uit in de nieuwe hoofdmap
- `sudo chroot pad/naar/nieuwe_hoofdmap commando` : voer het commando uit als nieuwe hoofdmap
- `sudo chroot --userspec gebruikersnaam_of_id:groep_naam_of_id pad/naar/nieuwe_hoofdmap` : gebruik een specifieke gebruiker en groep
