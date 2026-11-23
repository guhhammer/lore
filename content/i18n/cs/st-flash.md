# st-flash

Nahrává binární soubory do STM32 ARM Cortex mikrořadičů.

## Usage
- `st-flash read firmware.bin 0x8000000 4096` : přečíst 4096 bajtů ze zařízení začínající od 0x8000000
- `st-flash write firmware.bin 0x8000000` : zapsat firmware do zařízení začínající od 0x8000000
- `st-flash erase` : vymazat firmware ze zařízení

## Links
- More information: https://github.com/stlink-org/stlink/blob/testing/doc/man/st-flash.md
