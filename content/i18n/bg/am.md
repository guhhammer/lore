# am

Мениджър на дейности на Android.

## Usage
- `am start -n com.android.settings/.Settings` : стартиране на дейността с конкретен компонент и име на пакет name
- `am start -a android.intent.action.VIEW -d tel:123` : стартиране на действие action и предаване на data към него
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : стартиране на дейност, съответстваща на конкретно action и category
- `am to-uri -a android.intent.action.VIEW -d tel:123` : конвертиране на намерение в URI

## Links
- More information: https://developer.android.com/tools/adb#am
