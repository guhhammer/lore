# scoop bucket

Zarządzaj bucketami: repozytoriami Git zawierającymi pliki, które opisują sposób instalacji aplikacji przez Scoop. Jeśli Scoop nie zna lokalizacji bucketa, należy określić lokalizację jego repozytorium.

## Usage
- `scoop bucket list` : wyświetl wszystkie aktualnie używane buckety
- `scoop bucket known` : wyświetl wszystkie znane buckety
- `scoop bucket add nazwa` : dodaj znany bucket według jego nazwy
- `scoop bucket add nazwa https://example.com/repository.git` : dodaj nieznany bucket według jego nazwy i adresu URL repozytorium Git
- `scoop bucket rm nazwa` : usuń bucket według jego nazwy

## Links
- More information: https://github.com/ScoopInstaller/Scoop/wiki/Buckets
