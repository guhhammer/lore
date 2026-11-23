# mv

Přesouvá nebo přejmenuje soubory a adresáře.

## Usage
- `mv cesta/ke/zdroji cesta/k/cili` : přejmenovat soubor nebo adresář pokud cíl není už existující adresář
- `mv cesta/ke/zdroji cesta/k/existujicimu_adresari` : přesunout soubor nebo adresář do existujicího adresáře
- `mv cesta/ke/zdroji1 cesta/ke/zdroji2 ... cests/k/existujicimu_adresari` : přesunout více souborů do existujícího adresáře, zachová název souborů
- `mv -f cesta/ke/zdroji cesta/k/cili` : netázat se pro potrvzení před přepsáním existujících souborů
- `mv -i cesta/k/zdroji cests/k/cili` : zeptat se pro potvrzení interaktivně před přepsáním existujícíh souborů, bez ohledu na oprávnění souborů
- `mv -n cesta/ke/zdroji cests/k/cili` : nepřepisovat existující soubory v cíli
- `mv -v cests/ke/zdroji cesta/k/cili` : přesunout soubory ve verbózním režimu, zobrazující soubory potom co jsou přesunuty
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t cesta/k/cilovemu_adresari` : určit cílový adresář abyste mohli puoužít externí nástroje pro sebrání pohyblivých souborů

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
