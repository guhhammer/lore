# cp

Kopieer bestanden en mappen.

## Usage
- `cp pad/naar/bronbestand pad/naar/doelbestand` : kopieer een bestand naar een andere locatie
- `cp pad/naar/bronbestand pad/naar/doelmap` : kopieer een bestand naar een andere map, met behoud van de bestandsnaam
- `cp -r pad/naar/bronmap pad/naar/doelmap` : kopieer de inhoud van een map recursief naar een andere locatie (als de bestemming bestaat, wordt de map erin gekopieerd)
- `cp -vra pad/naar/bronmap pad/naar/doelmap` : kopieer een map recursief, in verbose modus (toont bestanden terwijl ze worden gekopieerd) en behoud rechten, tijdstempels en symlinks
- `cp -t pad/naar/doelmap pad/naar/bestand1 pad/naar/bestand2 ...` : kopieer meerdere bestanden tegelijk naar een map
- `cp -i *.ext pad/naar/doelmap` : kopieer alle bestanden met een specifieke extensie naar een andere locatie, in interactieve modus (vraagt de gebruiker om bevestiging voordat overschrijven plaatsvindt)
- `cp -L link pad/naar/doelmap` : volg symbolische links voordat je kopieert
- `cp --parents bron/pad/naar/bestand pad/naar/doelbestand` : gebruik het volledige pad van bronbestanden, maak eventuele missende tussenliggende mappen aan tijdens het kopiëren

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
