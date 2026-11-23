# screencap

Cattura uno screenshot di un display mobile. Questo comando può essere usato solo tramite `adb shell`.

## Usage
- `screencap percorso/del/file.png` : cattura uno screenshot
- `screencap -p` : stampa il contenuto del file su `stdout` come PNG
- `adb shell screencap -p >percorso/del/file.png` : cattura uno screenshot e salvalo tramite una connessione `adb`
- `screencap -h` : mostra l'aiuto

## Links
- More information: https://developer.android.com/tools/adb#screencap
