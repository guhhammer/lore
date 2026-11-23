# blackfire

Strumento di profilazione da linea di comando per PHP.

## Usage
- `blackfire config` : inizializza e configura il client Blackfire
- `blackfire agent` : lancia l'agent Blackfire
- `blackfire agent --socket="tcp://127.0.0.1:8307"` : lancia l'agent Blackfire su uno specifico socket
- `blackfire run php percorso/del/file.php` : lancia il profiler su uno specifico programma
- `blackfire --samples 10 run php percorso/del/file.php` : lancia il profiler e raccogli 10 campioni
- `blackfire --json run php percorso/del/file.php` : lancia il profiler e mostra i risultati in output come JSON
- `blackfire upload percorso/del/file` : carica un file del profiler sul servizio web di Blackfire
- `blackfire status` : mostra lo stato dei profili sul servizio web di Blackfire

## Links
- More information: https://www.blackfire.io/
