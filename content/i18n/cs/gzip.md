# gzip

Komprimuje/dekomprimuje soubory s `gzip` kompresí (LZ77).

## Usage
- `gzip cesta/k/souboru` : komprimovat soubor a nahradit jej `gzip` archivem
- `gzip -d cesta/k/souboru.gz` : dekomprimovat souboru a nahradit jej původní nekomprimovanou verzí
- `gzip -v cesta/k/souboru` : zobrazit název a procento zmenšení pro každý komprimovaný soubor
- `gzip -k cesta/k/souboru` : komprimovat soubor se zachováním původního souboru
- `gzip -c cesta/k/souboru >cesta/ke/komprimovanemu_souboru.gz` : komprimovat soubor a zadat název výstupního souboru
- `gzip -cd cesta/k/souboru.gz >cesta/k/nekomprimovanemu_souboru` : dekomprimovat `gzip` archiv a zadat název výstupního souboru
- `gzip -1..9 -c cesta/k/souboru >cesta/ke/komprimovanemu_souboru.gz` : upřesnit úroveň komprese. 1 je nejrychlejší (nízká komprese), 9 je nejpomalejší (vysoká komprese), 6 je výchozí
- `gzip -l cesta/k/souboru.txt.gz` : vypsat obsah komprimovaného souboru

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
