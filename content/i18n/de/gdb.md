# gdb

Der GNU Debugger.

## Usage
- `gdb ausführbare_datei` : debugge eine ausführbare Datei
- `gdb -p prozess_id` : binde einen Prozess an gdb
- `gdb -c kerndatei ausführbare_datei` : debugge mit einer Kerndatei
- `gdb -ex "befehle" ausführbare_datei` : führe angegebene Befehle beim Start von gdb aus
- `gdb --args ausführbare_datei argument1 argument2` : starte gdb und übergib Argumente an die ausführbare Datei

## Links
- More information: https://sourceware.org/gdb/current/onlinedocs/gdb#Invocation
