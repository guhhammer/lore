# am

Менеджер активностей Android.

## Usage
- `am start -n com.android.settings/.Settings` : запустить активность с указанным компонентом и именем (name) пакета
- `am start -a android.intent.action.VIEW -d tel:123` : запустить намерение с указанным действием (action) и передать в него данные (data)
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : запустить активность, соответствующую указанному действию (action) и категории (category)
- `am to-uri -a android.intent.action.VIEW -d tel:123` : преобразовать намерение в URI
- `am start -W -c android.intent.category.HOME -a android.intent.action.MAIN` : запустить домашнюю активность на эмуляторе или устройстве

## Links
- More information: https://developer.android.com/tools/adb#am
