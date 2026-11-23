# logcat

Scarica un registro di messaggi di sistema, comprese le stack traces quando si verifica un errore e i messaggi di informazioni di log dalle applicazioni.

## Usage
- `logcat` : visualizza i log di sistema
- `logcat -f percorso/del/file` : scrivi i log di sistema in un file
- `logcat --regex regex` : visualizza le linee che corrispondono a un `regex`
- `logcat --pid pid` : visualizza i log per uno specifico PID
- `logcat --pid $(pidof -s pacchetto)` : visualizza i log per il processo di uno specifico pacchetto

## Links
- More information: https://developer.android.com/tools/logcat
