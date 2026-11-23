# egrep

Vyhledat vzory v souborech pomocí rozšířených `regex` výrazů. Poznámka: Tento příkaz je aliasem pro `grep --extended-regexp`.

## Usage
- `egrep 'a+' cesta/k/souboru` : vyhledat jeden nebo více opakujících se znaků
- `egrep 'a?' cesta/k/souboru` : vyhledat jeden nebo žádný výskyt znaku (nepovinný výskyt)
- `egrep 'a{10}' cesta/k/souboru` : vyhledat 10 výskytů určitého znaku
- `egrep 'a{3,7}' cesta/k/souboru` : vyhledat 3 až 7 výskytů určitého znaku
- `egrep 'kocka|pes|mys' cesta/k/souboru` : vyhledat jednu z uvedených možností
- `egrep 'c(a|o|u)p' cesta/k/souboru` : vyhledat jednu z uvedených možností v rámci vetšího vzoru
- `egrep '(aeiou)+' cesta/k/souboru` : vyhledat skupinu znaků opakující se jednou nebo vícekrát
- `egrep [[:alnum:|:alpha:|:space:|...]] cesta/k/souboru` : vyhledat pomocí standardních tříd znaků (více info: <https://www.regular-expressions.info/posixbrackets.html>)

## Links
- More information: https://manned.org/egrep
