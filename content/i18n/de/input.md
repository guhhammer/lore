# input

Sende Eventcodes oder Touchscreengesten an ein Android-Gerät. Dieser Befehl kann nur mit `adb shell` verwendet werden.

## Usage
- `input keyevent eventcode` : sende einen Eventcode für die Eingabe eines einzelnen Zeichens an ein Gerät
- `input text "text"` : sende einen Text an ein Gerät (`%s` wird als Leerzeichen verwendet)
- `input tap x_pos y_pos` : sende ein einzelnes Tippen auf den Touchscreen an ein Gerät
- `input swipe x_start y_start x_ende y_ende dauer_in_ms` : sende ein Wischen über den Touchscreen an ein Gerät
- `input swipe x_pos y_pos x_pos y_pos dauer_in_ms` : sende ein langes Tippen auf den Touchscreen an ein Gerät
