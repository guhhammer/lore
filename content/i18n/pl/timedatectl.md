# timedatectl

Kontroluj datę i czas systemowy.

## Usage
- `timedatectl` : sprawdź aktualny czas zegara systemowego
- `timedatectl set-time "yyyy-MM-dd hh:mm:ss"` : bezpośrednio ustaw czas lokalny zegara systemowego
- `timedatectl list-timezones` : wyświetl dostępne strefy czasowe
- `timedatectl set-timezone strefa_czasowa` : ustaw systemową strefę czasową
- `timedatectl set-ntp on` : włącz synchronizację czasu poprzez Network Time Protocol (NTP)
- `timedatectl set-local-rtc 1` : zmień standard czasu zegara sprzętowego na czas lokalny

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/timedatectl.html
