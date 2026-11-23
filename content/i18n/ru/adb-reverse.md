# adb reverse

Android Debug Bridge Reverse: обратное соединение от эмулятора Android или подключенного устройства Android.

## Usage
- `adb reverse --list` : вывести список всех обратных соединений от эмуляторов и устройств
- `adb reverse tcp:удалённый_порт tcp:локальный_порт` : создать обратное соединение по TCP-порту от эмулятора или устройства до localhost
- `adb -s id_устройства reverse tcp:удалённый_порт tcp:локальный_порт` : создать обратное соединение по TCP-порту от определённого эмулятора или устройства (по ID устройства / sерийному номеру) до localhost
- `adb reverse --remove tcp:удалённый_порт` : удалить обратное соединение из эмулятора или устройства
- `adb reverse --remove-all` : удалить все обратные соединения на всех эмуляторах и устройствах

## Links
- More information: https://developer.android.com/tools/adb
