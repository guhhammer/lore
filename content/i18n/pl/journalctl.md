# journalctl

Przeszukaj dziennik systemd.

## Usage
- `journalctl -b -p 3` : wyświetl wszystkie wiadomości o priorytecie 3 (błędy) z tego rozruchu
- `journalctl --vacuum-time 2d` : usuń dzienniki starsze niż 2 dni
- `journalctl -f` : wyświetlaj nowe wiadomości (jak `tail -f` dla tradycyjnego sysloga)
- `journalctl -u jednostka` : wyświetl wszystkie wiadomości podanej jednostki
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID jednostka)` : wyświetl wiadomości podanej jednostki od czasu jej ostatniego uruchomienia
- `journalctl -S now|today|yesterday|tomorrow -U "YYYY-MM-DD HH:MM:SS"` : filtruj wiadomości w zakresie czasu (znacznik czasu lub symbol zastępczy, np. "yesterday")
- `journalctl _PID=pid` : wyświetl wszystkie wiadomości podanego procesu
- `journalctl ścieżka/do/pliku` : wyświetl wszystkie wiadomości podanego pliku wykonywalnego

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
