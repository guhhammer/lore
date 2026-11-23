# cp

Kopíruje soubory a adresáře.

## Usage
- `cp cesta/ke/zdrojovemu_souboru.ext cesta/k/cilovemu_souboru.ext` : zkopírovat soubor do jiné lokace
- `cp cesta/ke/zdrojovemu_adresari.ext cesta/k/nadrazenemu_adresari` : zkopírovat soubor do jiného adresáře, zachová název souboru
- `cp -r cesta/ke/zdrojovemu_adresari cesta/k/cilovemu_adresari` : rekurzivně zkopírovat obsah adresáře do jiné lokace (pokud cíl existuje, adresář se zkopíruje do něj)
- `cp -vra cesta/ke/zdrojovemu_adresari cesta/k/cilovemu_adresari` : zkopírovat adresář rekurzivně, ve verbózním režimu (zobrazí soubory jak jsou zkopírovány) a zachovat oprávnění, časové razítka, a symbolické odkazy
- `cp -t cesta/k/cilovemu_adresari cesta/k/souboru1 cesta/k/souboru2 ...` : zkopírovat více souborů do adresáře naráz
- `cp -i *.ext cesta/k/cilovemu_adresari` : zkopírovat všechny soubory se specifickou koncovkou do jiné lokace, v interaktívním režimu (zeptá se uživatele před přepsáním)
- `cp -L link cesta/k/cilovemu_adresari` : následovat symbolické linky před zkopírováním
- `cp --parents zdrojova/cesta/k/souboru cesta/k/cilovemu_souboru` : používat plnou cestu zdrojových souborů, automaticky vytvoří jakékoliv chybějící meziadresáře při kopírováním

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
