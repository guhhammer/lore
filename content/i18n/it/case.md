# case

Costrutto builtin di Bash per creare istruzioni condizionali a scelta multipla.

## Usage
- `case $COUNTRULE in words) wc --words README ;; lines) wc --lines README ;; esac` : confronta una variabile con stringhe letterali per decidere quale comando eseguire
- `case $COUNTRULE in wW]) wc --words README ;; lL]) wc --lines README ;; *) echo "cosa?" ;; esac` : combina pattern con |, usando * come pattern di fallback
- `case $ANIMAL in cat) echo "È un gatto" ;;& cat|dog) echo "È un gatto o un cane" ;;& *) echo "Fallback" ;; esac` : consente di confrontare più pattern
- `case $ANIMAL in cat) echo "È un gatto" ;& dog) echo "È un gatto o un cane passato al ramo successivo" ;& *) echo "Fallback" ;; esac` : continua con i comandi del pattern successivo senza ricontrollare il pattern
- `help case` : mostra l'aiuto

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-case
