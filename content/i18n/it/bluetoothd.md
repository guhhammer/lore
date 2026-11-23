# bluetoothd

Daemon per la gestione dei dispositivi Bluetooth.

## Usage
- `bluetoothd` : avvia il daemon
- `bluetoothd -n` : avvia il daemon, registrando su `stdout`
- `bluetoothd -f percorso/del/file` : avvia il daemon con un file di configurazione specifico (predefinito `/etc/bluetooth/main.conf`)
- `bluetoothd -d` : avvia il daemon con output verboso su `stderr`
- `bluetoothd -dpercorso/del/file1:percorso/del/file2:...` : avvia il daemon con output verboso proveniente da file specifici in bluetoothd o plugin

## Links
- More information: https://manned.org/bluetoothd
