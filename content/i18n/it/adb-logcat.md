# adb logcat

Estrae un log dei messaggi di sistema.

## Usage
- `adb logcat` : visualizza i log di sistema
- `adb logcat -e regex` : visualizza le righe che corrispondono a una `regex`
- `adb logcat tag:mode *:S` : visualizza i log per un tag in una modalità specifica (Verbose, Debug, Info, Warning, Error, Fatal, Silent), filtrando gli altri tag
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : visualizza i log per applicazioni React Native in modalità Verbose Silenzia gli altri tag
- `adb logcat *:W` : visualizza i log per tutti i tag con livello di priorità Warning e superiore
- `adb logcat --pid pid` : visualizza i log per un PID specifico
- `adb logcat --pid $(adb shell pidof -s pacchetto)` : visualizza i log per il processo di un pacchetto specifico
- `adb logcat -v color` : colora il log (di solito usato con filtri)

## Links
- More information: https://developer.android.com/tools/logcat
