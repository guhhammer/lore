# input

Invia codici evento o gesture touchscreen a un dispositivo Android. Questo comando può essere usato solo attraverso `abd shell`.

## Usage
- `input keyevent codice_evento` : invia un codice evento per un singolo carattere a un dispositivo Android
- `input text "testo"` : invia un testo a un dispositivo Android (`%s` rappresenta lo spazio)
- `input tap pos_x pos_y` : invia un singolo tap a un dispositivo Android
- `input swipe inizio_x inizio_y fine_x fine_y durata_in_ms` : invia una gesture di scorrimento a un dispositivo Android
- `input swipe pos_x pos_y pos_x pos_y durata_in_ms` : invia un tap lungo a un dispositivo Android usando una gesture di scorrimento
