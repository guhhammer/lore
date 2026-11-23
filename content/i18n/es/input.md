# input

Envía códigos de eventos o gestos de pantalla táctil a un dispositivo Android. Este comando solo se puede usar a través de `adb shell`.

## Usage
- `input keyevent codigo_evento` : envía un código de evento para un solo carácter a un dispositivo Android
- `input text "texto"` : envía un texto a un dispositivo Android (`%s` representa espacios)
- `input tap x_pos y_pos` : envía una pulsación a un dispositivo Android
- `input swipe x_start y_start x_end y_end duracion_en_ms` : envía un gesto de deslizamiento a un dispositivo Android
- `input swipe x_pos y_pos x_pos y_pos duracion_en_ms` : envía una pulsación larga a un dispositivo Android mediante un gesto de deslizamiento
