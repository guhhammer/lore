# logcat

Zrzut dziennika komunikatów systemowych, w tym śladów stosu po wystąpieniu błędu i komunikatów informacyjnych rejestrowanych przez aplikacje.

## Usage
- `logcat` : wyświetl logi systemowe
- `logcat -f ścieżka/do/pliku` : zapisz logi systemowe do pliku (file)
- `logcat --regex wyrażenie_regularne` : wyświetl linie pasujące do wyrażenia regularnego
- `logcat --pid pid` : wyświetl logi dla określonego PID
- `logcat --pid $(pidof -s pakiet)` : wyświetl logi dla procesu określonego pakietu

## Links
- More information: https://developer.android.com/tools/logcat
