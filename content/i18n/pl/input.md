# input

Wysyłaj kody zdarzeń lub gestów na ekranie dotykowym do urządzenia Android. Ta komenda może być używana tylko poprzez `adb shell`.

## Usage
- `input keyevent kod_zdarzenia` : wyślij kod zdarzenia dla pojedynczego znaku do urządzenia Android
- `input text "tekst"` : wyślij tekst do urządzenia z systemem Android (`%s` reprezentuje spacje)
- `input tap pozycja_x pozycja_y` : wyślij pojedyncze stuknięcie do urządzenia Android
- `input swipe x_start y_start x_koniec y_koniec czas_trwania_w_ms` : wyślij gest przesunięcia do urządzenia Android
- `input swipe pozycja_x pozycja_y pozycja_x pozycja_y czas_trwania_w_ms` : wyślij długie naciśnięcie do urządzenia Android za pomocą gestu przesunięcia
