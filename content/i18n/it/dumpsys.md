# dumpsys

Mostra informazioni sui servizi di sistema Android. Questo comando può essere usato solo tramite `adb shell`.

## Usage
- `dumpsys` : ottieni l'output diagnostico di tutti i servizi di sistema
- `dumpsys servizio` : ottieni l'output diagnostico di un servizio di sistema specifico
- `dumpsys -l` : elenca tutti i servizi di cui `dumpsys` può fornire informazioni
- `dumpsys servizio -h` : elenca gli argomenti specifici per un determinato servizio
- `dumpsys --skip servizio` : escludi un servizio specifico dall'output diagnostico
- `dumpsys -t 8` : specifica un timeout in secondi (predefinito: 10s)

## Links
- More information: https://developer.android.com/tools/dumpsys
