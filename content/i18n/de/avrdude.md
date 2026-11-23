# avrdude

Treiberprogramm für Atmel AVR Mikrocontroller-Programmierung.

## Usage
- `avrdude -p part_id -c programmer -U flash:r:pfad/zu/datei.hex:i` : schreibt den Speicherinhalt eines AVR-Mikrocontrollers in eine Datei
- `avrdude -p part_id -c programmer -U flash:w:pfad/zu/datei.hex` : schreibt den Inhalt einer Datei in einen AVR-Mikrocontroller
- `avrdude -p \?` : liste alle verfügbaren AVR-Geräte auf
- `avrdude -c \?` : liste alle verfügbaren AVR-Programmer auf
