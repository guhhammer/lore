# mysql

Narzędzie wiersza polecenia MySQL.

## Usage
- `mysql nazwa_bazy_danych` : połącz się z bazą danych
- `mysql -u użytkownik -p nazwa_bazy_danych` : połącz się z bazą danych, użytkownik zostanie poproszony o podanie hasła
- `mysql -h host_bazy_danych nazwa_bazy_danych` : połącz się z bazą danych na innym hoście
- `mysql -S ścieżka/do/gniazda.sock` : połącz się z bazą danych przez gniazdo Unix
- `mysql -e "source nazwa_pliku.sql" nazwa_bazy_danych` : wykonuj instrukcje SQL w pliku skryptu (plik wsadowy)
- `mysql <ścieżka/do/kopii_zapasowej.sql -u użytkownik -p nazwa_bazy_danych` : przywróć bazę danych z kopii zapasowej (użytkownik zostanie poproszony o podanie hasła)
- `mysql <ścieżka/do/kopii_zapasowej.sql -u użytkownik -p` : przywróć wszystkie bazy danych z kopii zapasowej (użytkownik zostanie poproszony o podanie hasła)

## Links
- More information: https://manned.org/mysql
