# am

Gerenciador de atividades do Android (Activity Manager).

## Usage
- `am start -n com.android.settings/.Settings` : inicia uma activity específica
- `am start -a android.intent.action.VIEW -d tel:123` : inicia uma activity e passa dados para ela
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : inicia uma activity correspondente a uma ação e categoria específicas
- `am to-uri -a android.intent.action.VIEW -d tel:123` : converte uma intent em uma URI

## Links
- More information: https://developer.android.com/tools/adb#am
