# esptool.py

Bootloader nástroj pro Espressif čipy (např. ESP8266).

## Usage
- `sudo esptool.py --port port --baud prenosova_rychlost write_flash 0x0 cesta/k/firmwaru.bin` : flashnout firmware soubor do ESP čipu s uvedeným portem a přenosovou rychlostí
- `sudo esptool.py --port port --baud prenosova_rychlost erase_flash` : smazat flash ESP čipu

## Links
- More information: https://docs.espressif.com/projects/esptool/en/latest/esp32/
