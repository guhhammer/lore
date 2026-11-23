# avrdude

Programa controlador para la programación de microcontroladores Atmel AVR.

## Usage
- `avrdude -p part_id -c programador -U flash:r:file.hex:i` : lee el microcontrolador AVR
- `avrdude -p part_id -c programador -U flash:w:file.hex` : escribe el microcontrolador AVR
- `avrdude -p \?` : lista de dispositivos AVR disponibles
- `avrdude -c \?` : lista de programadores AVR disponibles
