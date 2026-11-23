# csplit

Splits een bestand in stukken. Dit genereert bestanden genaamd `xx00`, `xx01` etc.

## Usage
- `csplit pad/naar/bestand 10` : splits een bestand in twee delen, waarbij het tweede deel begint op regel 10
- `csplit pad/naar/bestand 7 23` : splits een bestand in drie delen, waarbij de laatste delen beginnen op regel 7 en 23
- `csplit pad/naar/bestand 5 {*}` : start een nieuw deel op elke 5e regel (faalt als het aantal regels niet deelbaar is door 5)
- `csplit -k pad/naar/bestand 5 {*}` : start een nieuw deel op elke 5e regel, waarbij de fout bij niet-exacte delingen wordt genegeerd
- `csplit pad/naar/bestand 5 -f prefix` : splits een bestand boven regel 5 en gebruik een aangepaste prefix voor de uitvoerbestanden (standaard is `xx`)
- `csplit pad/naar/bestand /regex/` : splits een bestand boven de eerste regel die overeenkomt met de `regex`
