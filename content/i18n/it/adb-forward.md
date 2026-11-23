# adb forward

Inoltra le connessioni socket a un dispositivo Android o emulatore collegato.

## Usage
- `adb forward tcp:porta_locale tcp:porta_remota` : inoltra una porta TCP all'unico emulatore o dispositivo collegato
- `adb -s device_id forward tcp:porta_locale tcp:porta_remota` : inoltra una porta TCP a un emulatore o dispositivo specifico (per ID dispositivo / numero seriale)
- `adb forward --list` : elenca tutti gli inoltri
- `adb forward --remove tcp:porta_locale` : rimuove una regola di inoltro
- `adb forward --remove-all` : rimuove tutte le regole di inoltro

## Links
- More information: https://developer.android.com/tools/adb
