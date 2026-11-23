# zip

Zabaluje a komprimuje (archivuje) soubory do Zip archivu.

## Usage
- `zip -r cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : přidat soubory/adresáře do konkrétního archivu
- `zip -d cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : vymazat soubory/adresáře z konkrétního archivu
- `zip -r cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ... -x cesta/k/vyrazenych_souboru_nebo_adresaru` : archivovat soubory/adresáře kromě uvedených
- `zip -r -0..9 cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : archivovat soubory/adresáře s určitou úrovní komprimace (`0` - nejnižší, `9` - nejvyšší)
- `zip -re cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : vytvořit zašifrovaný archiv pod uvedeným heslem
- `zip -rs 3g cesta/ke/komprimovanemu.zip cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : archivovat soubory/složky do vícedílného rozděleného Zip archivu (např. 3GB díly)
- `zip -sf cesta/ke/komprimovanemu.zip` : vypsat obsah konkrétního archivu

## Links
- More information: https://manned.org/zip
