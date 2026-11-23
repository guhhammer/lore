# coproc

Bash ingebouwd commando voor het maken van interactieve asynchrone subshells.

## Usage
- `coproc { commando1; commando2; ...; }` : voer een subshell asynchroon uit
- `coproc naam { commando1; commando2; ...; }` : maak een coprocess met een specifieke naam
- `echo "invoer" >&"${naam[1]}"` : schrijf naar de `stdin` van een specifiek coprocess
- `read <&"${naam[0]}" variabele` : lees van de `stdout` van een specifiek coprocess
- `coproc naam { while read regel; do commando1; commando2; ...; done }` : maak een coprocess dat herhaaldelijk `stdin` leest en opdrachten op de invoer uitvoert
- `coproc naam { while read regel; do echo "$regel" | commando1 | commando2 | ... | cat /dev/fd/0; done }` : maak een coprocess dat herhaaldelijk `stdin` leest, voert een pipeline uit op de input en schrijf de output naar `stdout`
- `coproc BC { bc -l; }; echo "1/3" >&"${BC[1]}"; read <&"${BC[0]}" output; echo "$output"` : maak en gebruik een coprocess dat `bc` uitvoert

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Coprocesses
