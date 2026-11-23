# rm

Maže soubory nebo adresáře.

## Usage
- `rm cesta/k/souboru1 cesta/k/souboru2 ...` : smazat konkrétní soubory
- `rm -f cesta/k/souboru1 cesta/k/souboru2 ...` : smazat konkrétní soubory ignorující neexistující
- `rm -i cesta/k/souboru1 cesta/k/souboru2 ...` : smazat konkrétní soubory dotazující se interaktivně před každým smazáním
- `rm -v cesta/k/souboru1 cesta/k/souboru2 ...` : smazat konkrétní soubory vypisující informace o každém smazáním
- `rm -r cesta/k/souboru_ci_adresari1 cesta/k/souboru_ci_adresari2 ...` : smazat konkrétní soubory a adresáře rekurzivně
- `rm -d cesta/k/adresari` : smazat prázdné adresáře (tohle je považováno za bezpečnou metodu)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/rm-invocation.html
