# loginctl

Zarządzaj menedżerem logowania systemd.

## Usage
- `loginctl list-sessions` : wyświetl wszystkie aktualne sesje
- `loginctl show-session id_sesji --all` : wyświetl wszystkie właściwości podanej sesji
- `loginctl show-user nazwa_użytkownika` : wyświetl wszystkie właściwości podanego użytkownika
- `loginctl show-user nazwa_użytkownika --property nazwa_właściwości` : wyświetl podaną właściwość użytkownika
- `loginctl list-users -H nazwa_hosta` : uruchom operację `loginctl` na zdalnym hoście

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/loginctl.html
