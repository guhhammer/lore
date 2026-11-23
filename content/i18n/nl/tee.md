# tee

Lees van `stdin` en schrijf naar `stdout` en bestanden (of commando's).

## Usage
- `echo "voorbeeld" | tee pad/naar/bestand` : kopieer `stdin` naar elk bestand en ook naar `stdout`
- `echo "voorbeeld" | tee -a pad/naar/bestand` : voeg toe aan de opgegeven bestanden, overschrijf niet
- `echo "voorbeeld" | tee /dev/tty | xargs printf "[%s]"` : toon `stdin` naar de terminal en leid het ook door naar een ander programma voor verdere verwerking
- `echo "voorbeeld" | tee >(xargs mkdir) >(wc -c)` : maak een map genaamd "voorbeeld", tel het aantal tekens in "voorbeeld" en schrijf "voorbeeld" naar de terminal

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tee-invocation.html
