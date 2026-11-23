# systemd-analyze

Analizuj i debuguj menedżera systemu. Wyświetl szczegóły dotyczące czasiu procesu uruchamiania jednostek (usług, punktów montowania, urządzeń, gniazd).

## Usage
- `systemd-analyze blame` : wyświetl wszystkie uruchomione jednostki, uporządkowane według czasu ich inicjalizacji
- `systemd-analyze critical-chain` : wyświetl drzewo krytycznego czasowo łańcucha jednostek
- `systemd-analyze plot >ścieżka/do/pliku.svg` : utwórz plik SVG pokazujący kiedy każda usługa wystartowała, zaznaczając czas wykorzystany na inicjalizację
- `systemd-analyze dot | dot -Tsvg >ścieżka/do/pliku.svg` : sporządź wykres zależności i przekonwertuj go do pliku SVG
- `systemd-analyze security` : wyświetl wyniki bezpieczeństwa działających jednostek

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-analyze.html
