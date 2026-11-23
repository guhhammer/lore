# unzip

Extrahuje soubory/adresáře ze Zipových archivů.

## Usage
- `unzip cesta/k/archivu1.zip cesta/k/archivu2.zip ...` : extrahovat všechny soubory/adresáře ze specifického archivu do aktuálního adresáře
- `unzip cesta/k/archivu1.zip cesta/k/archivu2.zip ... -d cesta/k/vystupu` : extrahovat soubory/adresáře z archivu do konkrétní cesty
- `unzip -c cesta/k/archivu1.zip cesta/k/archivu2.zip ...` : extrahovat soubory/adresáře z archivů do `stdout` spolu s názvy souborů
- `unzip -O gbk cesta/k/archivu1.zip cesta/k/archivu2.zip ...` : extrahovat archiv vytvořený na Windows, obsahujicí soubory s názvy souborů mimo ASCII (např. Čínské nebo Japonské znaky)
- `unzip -l cesta/k/archivu.zip` : vypsat obsah konkrétního archivu bez extrahování
- `unzip -j cesta/k/archivu.zip cesta/k/souboru1_v_archivu cesta/k/souboru2_v_archivu ...` : extrahovat konkrétní soubor z archivu

## Links
- More information: https://manned.org/unzip
