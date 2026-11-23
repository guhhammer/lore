# fossil commit

Commit bestanden naar een Fossil-repository.

## Usage
- `fossil ci` : maak een nieuwe versie met alle aanpassingen in de huidige checkout; de gebruiker zal gevraagd worden voor een opmerking
- `fossil ci -m "opmerking"` : maak een nieuwe versie met alle aanpassingen in de huidige checkout en maak gebruik van de gespecificeerde opmerking
- `fossil ci -M pad/naar/commit_bericht_bestand` : maak een nieuwe versie met alle aanpassingen in de huidige checkout met een opmerking ingelezen vanaf een specifiek bestand
- `fossil ci pad/naar/bestand1 pad/naar/bestand2 ...` : maak een nieuwe versie met aanpassingen van de gespecificeerde bestanden; de gebruiker zal gevraagd worden voor een opmerking

## Links
- More information: https://fossil-scm.org/home/help/commit
