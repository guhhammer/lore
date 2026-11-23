# avrdude

Program ovladače pro programování Atmel AVR mikrořadičů.

## Usage
- `avrdude -p part_id -c id_programatora -U flash:r:soubor.hex:i` : prečíst flash ROM AVR mikrořadiče s konkrétním ID součástky
- `avrdude -p part_id -c programator -U flash:w:file.hex` : zapsat do flash ROM AVR mikrořadiče
- `avrdude -p \?` : vypsat dostupné AVR zařízení
- `avrdude -c \?` : vypsat dostupné AVR programátory
