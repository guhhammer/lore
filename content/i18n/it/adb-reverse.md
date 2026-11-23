# adb reverse

Inverte le connessioni socket da un dispositivo Android o emulatore collegato.

## Usage
- `adb reverse --list` : elenca tutte le connessioni socket inverse da emulatori e dispositivi
- `adb reverse tcp:porta_remota tcp:porta_locale` : inverte una porta TCP dall'unico emulatore o dispositivo collegato verso localhost
- `adb -s device_id reverse tcp:porta_remota tcp:porta_locale` : inverte una porta TCP da un emulatore o dispositivo specifico (per ID dispositivo / numero seriale) verso localhost
- `adb reverse --remove tcp:porta_remota` : rimuove una connessione socket inversa da un emulatore o dispositivo
- `adb reverse --remove-all` : rimuove tutte le connessioni socket inverse da tutti gli emulatori e dispositivi

## Links
- More information: https://developer.android.com/tools/adb
