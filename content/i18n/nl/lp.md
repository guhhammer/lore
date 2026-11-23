# lp

Print bestanden.

## Usage
- `echo "test" | lp` : print de output van een commando naar de standaard printer (bekijk het `lpstat` commando)
- `lp pad/naar/bestand` : print een bestand naar de standaard printer
- `lp -d printer_naam pad/naar/bestand` : print een bestand naar een printer met naam (bekijk het `lpstat` commando)
- `lp -n n pad/naar/bestand` : print `n` kopieën van een bestand naar de standaard printer
- `lp -P 1,3-5,16 pad/naar/bestand` : print alleen specifieke pagina's naar de standaard printer (print pagina's 1, 3-5 en 16)
- `lp -i taak_id -H resume` : hervat het printen van een taak

## Links
- More information: https://manned.org/lp
