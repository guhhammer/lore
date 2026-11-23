# am

Androids Aktivitäten-Manager.

## Usage
- `am start -n com.android.settings/.Settings` : starte eine bestimmte Aktivität
- `am start -a android.intent.action.VIEW -d tel:123` : starte eine Aktivität und übergib ihr Daten
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : starte eine Aktivität, auf die eine bestimmte Aktion und Kategorie zutrifft
- `am to-uri -a android.intent.action.VIEW -d tel:123` : konvertiere ein bestimmtes Ziel in einen URI

## Links
- More information: https://developer.android.com/tools/adb#am
