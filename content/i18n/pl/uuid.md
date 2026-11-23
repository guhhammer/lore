# uuid

Twórz i dekoduj uniwersalne identyfikatory (UUID).

## Usage
- `uuid` : stwórz UUIDv1 (oparte o zegar systemowy i - jeśli dostępne - adres sprzętowy)
- `uuid -v 4` : stwórz UUIDv4 (losowy)
- `uuid -v 4 -n ilość_uuid` : stwórz wiele UUIDv4 na raz
- `uuid -v 4 -F BIN|STR|SIV` : stwórz UUIDv4 w konkretnym formacie
- `uuid -v 4 -o ścieżka/do/pliku` : stwórz UUIDv4 i zapisz do pliku
- `uuid -v 5 ns:DNS|URL|OID|X500 nazwa_obiektu` : stwórz UUIDv5 (oparty o podaną nazwę obiektu) w przestrzeni nazw
- `uuid -d uuid` : dekoduj podany UUID

## Links
- More information: https://manned.org/uuid
