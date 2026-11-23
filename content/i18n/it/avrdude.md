# avrdude

Driver per il programmatore di microcontrollori Atmel AVR.

## Usage
- `avrdude -p part_id -c id_programmatore -U flash:r:file.hex:i` : leggi dal microcontrollore AVR
- `avrdude -p part_id -c id_programmatore -U flash:w:file.hex` : scrivi sul microcontrollore AVR
- `avrdude -p \?` : elenca dispositivi AVR disponibili
- `avrdude -c \?` : elenca programmatori AVR disponibili
