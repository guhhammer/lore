# pm

Менеджер пакетов Android.

## Usage
- `pm list packages` : показать список установленных пакетов
- `pm install /путь/к/приложению.apk` : установить пакет приложения по указанному пути
- `pm uninstall пакет` : удалить пакет с устройства
- `pm clear пакет` : очистить все данные приложения для пакета
- `pm enable пакет_или_класс` : включить пакет или компонент
- `pm disable-user пакет_или_класс` : отключить пакет или компонент
- `pm grant пакет android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : предоставить разрешение приложению
- `pm revoke пакет android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : отозвать разрешение у приложения

## Links
- More information: https://developer.android.com/tools/adb#pm
