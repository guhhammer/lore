# printf

Formatteer en toon tekst.

## Usage
- `printf "%s\n" "Hallo wereld"` : toon een tekstbericht
- `printf "%s\n" string1 string2 ...` : toon een geformatteerd bericht meerdere keren met verschillende strings
- `printf "\e[1;34m%.3d\e[0m\n" 42` : toon een geheel getal in vetgedrukt blauw
- `printf "\u20AC %.2f\n" 123.4` : toon een drijvend-komma getal met het Unicode euroteken
- `printf "var1: %s\tvar2: %s\n" "$VAR1" "$VAR2"` : toon een tekstbericht samengesteld met omgevingsvariabelen
- `printf -v mijnvar "Dit is %s = %d\n" "een jaar" 2016` : sla een geformatteerd bericht op in een variabele (werkt niet in Zsh)
- `printf "hex=%x octal=%o scientific=%e\n" 0xFF 0377 100000` : toon een hexadecimaal, octaal en wetenschappelijk getal
