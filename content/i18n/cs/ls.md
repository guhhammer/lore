# ls

Vypisuje obsah adresáře.

## Usage
- `ls -1` : vypsat soubory samostatně na každý řádek
- `ls -a` : vypsat všechny soubory, včetně skrytých souborů
- `ls -F` : vypsat všechny soubory s koncovým znakem který značí typ souboru (adresář/, symbolický_link@, spustitelný*, ...)
- `ls -la` : vypsat všechny soubory s dlouhým formátem (oprávnění, vlastnictví, velikost, a datum změny)
- `ls -lh` : vypsat všechny soubory s dlouhým formátem s velikostí v lidsky-čitelných jednotkách (KiB, MiB, GiB)
- `ls -lSR` : vypsat všechny soubory s dlouhým formátem, seřazené podle velikosti (sestupně) rekurzivně
- `ls -ltr` : vypsat všechny soubory s dlouhým formátem, seřazené podle času změny v obráceném pořadí (nejstarší první)
- `ls -d */` : vypsat pouze adresáře

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
