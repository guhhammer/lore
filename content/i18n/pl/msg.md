# msg

Wyślij wiadomość do wybranego użytnownika lub sesji.

## Usage
- `msg nazwa_użytkownika|nazwa_sesji|identyfikator_sesji wiadomość` : wysyła wiadomość do użytkownika lub sesji
- `echo "wiadomość" | msg nazwa_użytkownika|nazwa_sesji|identyfikator_sesji` : wyślij wiadomość ze standardowego wejścia
- `msg /server:nazwa_serwera nazwa_użytkownika|nazwa_sesji|identyfikator_sesji` : wyślij wiadomość to zdalnej maszyny
- `msg *` : wyślij wiadomość do wszystkich użytkowników aktualnej maszyny
- `msg /time:seconds` : wyślij wiadomość z opóźnieniem
