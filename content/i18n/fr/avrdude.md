# avrdude

Pilotes pour programmer les microcontrôleurs Atmel AVR.

## Usage
- `avrdude -p part_id -c programmeur -U flash:r:fichier.hex:i` : lire le contenu du microcontrôleur AVR
- `avrdude -p part_id -c programmeur -U flash:w:fichier.hex` : programme le microcontrôleur AVR
- `avrdude -p \?` : affiche les appareils AVR disponibles
- `avrdude -c \?` : affiche les programmeurs AVR disponibles
