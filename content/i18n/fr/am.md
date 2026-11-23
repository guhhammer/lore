# am

Manager d'activité Android.

## Usage
- `am start -n com.android.settings/.Settings` : commence une activité spécifique
- `am start -a android.intent.action.VIEW -d tel:123` : commence une activité et insère de la donnée
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : commence une activité qui correspond à une action et une catégorie spécifique
- `am to-uri -a android.intent.action.VIEW -d tel:123` : convertis une intention en URI

## Links
- More information: https://developer.android.com/tools/adb#am
