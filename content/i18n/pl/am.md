# am

Menedżer aktywności Android.

## Usage
- `am start -n com.android.settings/.Settings` : rozpocznij aktywność z określoną nazwą komponentu i pakietu
- `am start -a android.intent.action.VIEW -d tel:123` : rozpocznij akcję intencji i przekaż do niej dane
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : rozpocznij aktywność dopasowaną do konkretnej akcji i kategorii (category)
- `am to-uri -a android.intent.action.VIEW -d tel:123` : konwertuj intencję na URI
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : uruchom aktywność ekranu głównego na emulatorze lub urządzeniu

## Links
- More information: https://developer.android.com/tools/adb#am
