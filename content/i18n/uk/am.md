# am

Менеджер активності Android.

## Usage
- `am start -n com.android.settings/.Settings` : почати специфічну активність
- `am start -a android.intent.action.VIEW -d tel:123` : почати активність та передайте дані у неї
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : почати активність яка є специфічною дією та категорією
- `am to-uri -a android.intent.action.VIEW -d tel:123` : перетворити значення в посилання

## Links
- More information: https://developer.android.com/tools/adb#am
