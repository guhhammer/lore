# split

Split een bestand in stukken.

## Usage
- `split -l 10 pad/naar/bestand` : split een bestand, elk deel heeft 10 regels (behalve het laatste deel)
- `split -n 5 pad/naar/bestand` : split een bestand in 5 bestanden. Het bestand wordt zo gesplitst dat elk deel dezelfde grootte heeft (behalve het laatste deel)
- `split -b 512 pad/naar/bestand` : split een bestand met 512 bytes in elk deel (behalve het laatste deel; gebruik 512k voor kilobytes en 512m voor megabytes)
- `split -C 512 pad/naar/bestand` : split een bestand met maximaal 512 bytes in elk deel zonder regels te breken
- `gzip -cd pad/naar/gecomprimeerd_bestand.gz | split -l 1000 - pad/naar/uitvoer` : split in meerdere bestanden vanuit `stdin`
