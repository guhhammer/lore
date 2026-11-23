# date

Setze die Systemzeit oder zeige sie an.

## Usage
- `date +%c` : zeige das aktuelle Datum im Format der eingestellten Locale an
- `date -u +%Y-%m-%dT%H:%M:%SZ` : zeige das aktuelle Datum in koordinierter Weltzeit (UTC) im ISO 8601-Format an
- `date +%s` : zeige das aktuelle Datum in Unixzeit (vergangene Sekunden seit der Unix-Epoche) an
- `date -d @1473305798` : konvertiere ein in Unixzeit gegebenes Datum zum Standardformat
- `date -d "2018-09-01 00:00" +%s -u` : konvertiere ein gegebenes Datum zu Unixzeit
- `date --rfc-3339 s` : zeige das aktuelle Datum im RFC-3339 Format (`YYYY-MM-DD hh:mm:ss+TZ`) an
- `date 093023592021.59` : setze das aktuelle Datum im Format `MMDDhhmmYYYY.ss` (`YYYY` und `.ss` sind optional)
- `date +%V` : zeige die aktuelle ISO-Wochenzahl an

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
