# adb

Android Debug Bridge: comunica con un'istanza di un emulatore Android o con dispositivi Android connessi. Alcuni sottocomandi come `shell` hanno la propria documentazione di utilizzo.

## Usage
- `adb start-server` : controlla se il processo server adb è in esecuzione e avvialo
- `adb kill-server` : termina il processo server adb
- `adb shell` : avvia una shell remota nell'istanza dell'emulatore/dispositivo target
- `adb install -r percorso/del/file.apk` : installa un'applicazione Android su un emulatore/dispositivo
- `adb pull percorso/del/device_file_o_directory percorso/della/directory_di_destinazione_locale` : copia un file/directory dal dispositivo target
- `adb push percorso/del/file_locale_o_directory percorso/della/directory_di_destinazione_device` : copia un file/directory sul dispositivo target
- `adb devices` : elenca tutti i dispositivi connessi
- `adb -s device_id shell` : specifica quale dispositivo deve ricevere i comandi se ci sono più dispositivi

## Links
- More information: https://developer.android.com/tools/adb
