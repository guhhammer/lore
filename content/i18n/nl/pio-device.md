# pio device

Beheer en monitor PlatformIO apparaten.

## Usage
- `pio device list` : toon alle beschikbare seriële poorten
- `pio device list --logical` : toon alle beschikbare logische apparaten
- `pio device monitor` : start een interactieve apparaatmonitor
- `pio device monitor -p /dev/ttyUSBX` : start een interactieve apparaatmonitor en luister naar een specifieke poort
- `pio device monitor -b 57600` : start een interactieve apparaatmonitor en stel een specifieke baud in (standaard is 9600)
- `pio device monitor --eol CRLF|CR|LF` : start een interactieve apparaatmonitor en stel een specifiek EOL-karakter in (standaard is `CRLF`)
- `<Ctrl t>` : ga naar het menu van de interactieve apparaatmonitor

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/device/
