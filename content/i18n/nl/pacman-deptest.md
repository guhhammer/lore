# pacman-deptest

Controleer elke opgegeven afhankelijkheid en retourneer een lijst met afhankelijkheden die momenteel niet zijn voldaan op het systeem.

## Usage
- `pacman -T pakket1 pakket2 ...` : toon de pakketnamen van de afhankelijkheden welke niet geïnstalleerd zijn
- `pacman -T "bash>=5"` : controleer of het geïnstalleerde pakket voldoet aan de gegeven minimale versie
- `pacman -T "bash>5"` : controleer of een latere versie van een pakket is geïnstalleerd
- `pacman -Th` : toon de help

## Links
- More information: https://manned.org/pacman.8
