# adb install

Android Debug Bridge Install: Invia pacchetti ad un emulatore Android od ad un dispositivo Android connesso.

## Usage
- `adb install percorso/del/file.apk` : invia un'applicazione Android ad un emulatore/dispositivo
- `adb -s serial_number install percorso/del/file.apk` : invia un'applicazione Android ad un emulatore/dispositivo specifico (sovrascrive `$ANDROID_SERIAL`)
- `adb install -r percorso/del/file.apk` : reinstalla una applicazione esistente, preservandone i dati
- `adb install -d percorso/del/file.apk` : invia un'applicazione Android consentendo downgrade del codice versione (solo pacchetti debuggabili)
- `adb install -g percorso/del/file.apk` : grantisce tutti i permessi elencati nel manifest dell'applicazione
- `adb install --fastdeploy percorso/del/file.apk` : aggiorna rapidamente un pacchetto installato aggiornando solamente le parti dell'APK che sono cambiate

## Links
- More information: https://developer.android.com/tools/adb
