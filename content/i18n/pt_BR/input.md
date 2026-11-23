# input

Envia códigos de evento ou gestos de toque para um dispositivo Android. Esse comando só pode ser usado através de `adb shell`.

## Usage
- `input keyevent codigo_de_evento` : envia um código de evento de um caractere para um dispositivo Android
- `input text "texto"` : envia texto para um dispositivo Android (`%s` representa espaços)
- `input tap x_pos y_pos` : envia um único toque para um dispositivo Android
- `input swipe x_inicio y_inicio x_fim y_fim duração_em_ms` : envia um gesto de deslizar para um dispositivo Android
- `input swipe x_pos y_pos x_pos y_pos duração_em_ms` : envia um pressionamento longo usando gesto de deslizar para um dispositivo Android
