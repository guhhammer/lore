# date

Stel de systeemdatum in of toon deze.

## Usage
- `date +%c` : toon de huidige datum in het standaardformaat van de locale
- `date -u +%Y-%m-%dT%H:%M:%SZ` : toon de huidige datum in UTC, in het ISO 8601-formaat
- `date +%s` : toon de huidige datum als een Unix timestamp (seconden sinds de Unix-epoch)
- `date -d @1473305798` : converteer een datum gespecificeerd als een Unix timestamp naar het standaard formaat
- `date -d "2018-09-01 00:00" +%s -u` : converteer een opgegeven datum naar het Unix timestamp formaat
- `date --rfc-3339 date|seconds|ns` : toon de huidige datum in het RFC-3339 formaat (`YYYY-MM-DD hh:mm:ss+TZ`) en specificeer de precisie ervan
- `sudo date 093023592021.59` : stel de huidige datum in met het formaat `MMDDhhmmYYYY.ss` (`YYYY` en `.ss` zijn optioneel)
- `date +%V` : toon het huidige ISO-weeknummer

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
