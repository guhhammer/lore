# bitcoin-cli

Klient konsolowy do interakcji z demonem Bitcoina poprzez zapytania RPC. Używa konfiguracji zdefiniowanej w `bitcoin.conf`.

## Usage
- `bitcoin-cli sendtoaddress "adres" ilość` : wyślij transakcję na dany adres
- `bitcoin-cli generate ilość_bloków` : wygeneruj jeden lub więcej bloków
- `bitcoin-cli getwalletinfo` : wyświetl informacje o portfelu
- `bitcoin-cli listunspent` : wyświetl wszystkie poprzednie transakcje dostępne do opłacenia transakcji wychodzących
- `bitcoin-cli dumpwallet "ścieżka/do/pliku"` : wyeksportuj dane portfela do pliku tekstowego

## Links
- More information: https://en.bitcoin.it/wiki/Running_Bitcoin#Command-line_arguments
