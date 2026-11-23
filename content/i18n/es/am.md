# am

Administrador de actividades de Android.

## Usage
- `am start -n com.android.settings/.Settings` : inicia una actividad específica
- `am start -a android.intent.action.VIEW -d tel:123` : inicia una actividad y le suministra datos
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : inicia una actividad que coincide con una acción y categoría específicas
- `am to-uri -a android.intent.action.VIEW -d tel:123` : convierte una intención en una URI

## Links
- More information: https://developer.android.com/tools/adb#am
