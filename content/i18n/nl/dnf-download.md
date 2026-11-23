# dnf download

Download RPM-pakketten uit de DNF-repositories. Niet standaard inbegrepen bij `dnf`, maar ondersteund via `dnf-plugins-core`.

## Usage
- `dnf download pakket` : download de recentste versie van een pakket naar de huidige map
- `dnf download pakket --destdir pad/naar/map` : download een pakket naar een specifieke map (de map moet bestaan)
- `dnf download --url pakket` : toon de URL waar het RPM-pakket kan worden gedownload

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/download.html
