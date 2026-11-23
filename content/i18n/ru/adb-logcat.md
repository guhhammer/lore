# adb logcat

Выводить лог системных сообщений.

## Usage
- `adb logcat` : показать системные логи
- `adb logcat -e регулярное_выражение` : показать строки, соответствующие `regex`
- `adb logcat тег:режим *:S` : показать логи для тега в определённом режиме (Verbose, Debug, Info, Warning, Error, Fatal, Silent), фильтруя остальные теги
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : показать логи приложений React Native в подробном (Verbose) режиме, подавив (Silent) остальные теги
- `adb logcat *:W` : показать логи для всех тегов с уровнем приоритета Warning и выше
- `adb logcat --pid id_процесса` : показать логи для конкретного PID
- `adb logcat --pid $(adb shell pidof -s пакет)` : показать логи для процесса конкретного пакета
- `adb logcat -v color` : раскрасить лог (обычно используется с фильтрами)

## Links
- More information: https://developer.android.com/tools/logcat
