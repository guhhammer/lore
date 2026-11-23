# chpass

Dodaj lub zmień informacje w bazie danych użytkowników, w tym powłokę logowania i hasło.

## Usage
- `chpass` : dodaj lub zmień informacje w bazie danych użytkowników dla bieżącego użytkownika w sposób interaktywny
- `chpass -s ścieżka/do/powłoki` : ustaw określoną powłokę (z ang. shell) logowania dla bieżącego użytkownika
- `chpass -s ścieżka/do/powłoki nazwa_użytkownika` : ustaw powłokę (z ang. shell) logowania dla określonego użytkownika
- `su -c 'chpass -e czas nazwa_użytkownika'` : zmień czas wygaśnięcia (z ang. expire) konta (w sekundach od daty początku epoki, UTC)
- `su -c 'chpass -p zaszyfrowane_hasło nazwa_użytkownika'` : zmień hasło użytkownika
- `su -c 'chpass -h nazwa_hosta nazwa_użytkownika'` : określ nazwę hosta lub adres serwera NIS do zapytania
- `su -c 'chpass -d domain nazwa_użytkownika'` : określ konkretną domenę NIS (domyślnie nazwa domeny systemowej)

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass
