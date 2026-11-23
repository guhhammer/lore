# avrdude

Driver program for Atmel AVR microcontrollers programming.

## Usage
- `avrdude -p part_id -c programmer_id -U flash:r:file.hex:i` : read the flash ROM of an AVR microcontroller with a specific part ID
- `avrdude -p part_id -c programmer -U flash:w:file.hex` : write to the flash ROM AVR microcontroller
- `avrdude -p \?` : list available AVR devices
- `avrdude -c \?` : list available AVR programmers

## Categories
development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/avrdude
