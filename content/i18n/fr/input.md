# input

Envoie à un appareil Android des codes événements ou des gestes d'écran tactile. Cette commande peut être utilisée uniquement depuis `adb shell`.

## Usage
- `input keyevent event_code` : envoie un code événement (un seul caractère) à un appareil Android
- `input text "text"` : envoie du texte à un appareil Android (`%s` représentant les espaces)
- `input tap x_pos y_pos` : envoie un tapotement (tap) à un appareil Android
- `input swipe x_start y_start x_end y_end duration_in_ms` : envoie un mouvement de swipe à un appareil Android
- `input swipe x_pos y_pos x_pos y_pos duration_in_ms` : envoie un appui long à un appareil Android en utilisant un mouvement de swipe
