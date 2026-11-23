# tar

Archivovací nástroj. Často kombinován s komprimovací metodou, například `gzip` nebo `bzip2`.

## Usage
- `tar cf cesta/k/cili.tar cesta/k/souboru1 cesta/k/souboru2 ...` : vytvořit archiv a zapsat jej do souboru
- `tar czf cesta/k/cili.tar.gz cesta/k/souboru1 cesta/k/souboru2 ...` : vytvořit gzip archiv a zapsat jej do souboru
- `tar czf cesta/k/cili.tar.gz -C cesta/k/adresari .` : vytvořit gzip (komprimovaný) archiv z adresáře pomocí relativních cest
- `tar xvf cesta/ke/zdroji.tar.ext` : extrahovat (komprimovaný) archiv do aktuálního adresáře verbózně
- `tar xf cesta/ke/zdroji.tar.ext -C cesta/k/adresari` : extrahovat (komprimovaný) archiv do cílového adresáře
- `tar caf cesta/k/cili.tar.xz cesta/k/souboru1 cesta/k/souboru2 ...` : vytvořit komprimovaý archiv, zapsat jej do souboru a automaticky určit program pro kompresi pomocí přípony souboru
- `tar tvf cesta/ke/zdroji.tar` : vypsat obsah souboru tar verbózně
- `tar xf cesta/k/source.tar --wildcards "*.html"` : extrahovat soubory, které se shodují se vzorem z archivu

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
