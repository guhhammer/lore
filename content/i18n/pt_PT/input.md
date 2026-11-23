# input

Envia códigos de eventos ou toques no ecrã para um dispositivo Android. Este comando só pode ser usado com a `adb shell`.

## Usage
- `input keyevent codigo_de_evento` : envia um código de evento de um caractere para um dispositivo Android
- `input text "texto"` : envia texto para um dispositivo Android (`%s` representa espaços)
- `input tap x_pos y_pos` : envia um único toque para um dispositivo Android
- `input swipe x_inicio y_inicio x_fim y_fim duração_em_ms` : envia um gesto de deslizar para um dispositivo Android
- `input swipe x_pos y_pos x_pos y_pos duração_em_ms` : envia um toque prolongado usando um gesto de deslize para um dispositivo Android
