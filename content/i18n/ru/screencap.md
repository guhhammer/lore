# screencap

Делать снимки экрана мобильного дисплея. Примечание: эту команду можно использовать только через `adb shell`.

## Usage
- `screencap путь/к/изображению.png` : сделать снимок экрана
- `screencap -p` : вывести содержимое файла в `stdout` в формате PNG
- `adb shell screencap -p >путь/к/изображению.png` : сделать снимок экрана и сохранить его через соединение `adb`
- `screencap -h` : показать справку

## Links
- More information: https://developer.android.com/tools/adb#screencap
