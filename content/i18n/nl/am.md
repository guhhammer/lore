# am

Android-activiteitenmanager.

## Usage
- `am start -n com.android.settings/.Settings` : start de activiteit met een specifiek component en pakketnaam
- `am start -a android.intent.action.VIEW -d tel:123` : start een intent-actie en geef er data aan door
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : start een activiteit die overeenkomt met een specifieke actie en categorie
- `am to-uri -a android.intent.action.VIEW -d tel:123` : converteer een intent naar een URI
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : start de home-activiteit op een emulator of apparaat

## Links
- More information: https://developer.android.com/tools/adb#am
