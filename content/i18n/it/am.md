# am

Gestione delle attività di Android.

## Usage
- `am start -n com.android.settings/.Settings` : avvia l'activity con un componente specifico e nome del pacchetto
- `am start -a android.intent.action.VIEW -d tel:123` : avvia un'intent action e passa dati ad essa
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : avvia un'activity che corrisponde a un'azione e una categoria specifica
- `am to-uri -a android.intent.action.VIEW -d tel:123` : converte un intent in un URI
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : avvia l'activity home su un emulatore o dispositivo

## Links
- More information: https://developer.android.com/tools/adb#am
