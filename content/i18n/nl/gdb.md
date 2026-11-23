# gdb

De GNU Debugger.

## Usage
- `gdb pad/naar/uitvoerbaar_bestand` : debug een uitvoerbaar bestand
- `gdb -p proces_id` : koppel een proces aan `gdb`
- `gdb -c pad/naar/core pad/naar/uitvoerbaar_bestand` : debug met een core-bestand
- `gdb -ex "commando's" pad/naar/uitvoerbaar_bestand` : voer bepaalde GDB-commando's uit bij het starten
- `gdb --args pad/naar/uitvoerbaar_bestand argument1 argument2 ...` : start `gdb` en geef argumenten mee aan het uitvoerbaar bestand
- `gdb -c pad/naar/core pad/naar/uitvoerbaar_bestand -iex 'set debuginfod enabled on' -iex 'set pagination off' -ex bt` : sla `debuginfod`- en paginationprompts over en toon de backtrace

## Links
- More information: https://sourceware.org/gdb/current/onlinedocs/gdb#Invocation
