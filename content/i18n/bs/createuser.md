# createuser

Napravi PostgreSQL koristnika (uloga).

## Usage
- `createuser --interactive korisničko_ime` : stvori koristnika interaktivno
- `createuser korisničko_ime` : stvori koristnika bez posebnih prava
- `createuser -s korisničko_ime` : stvori superkoristnika
- `createuser -d -r -P korisničko_ime` : stvori koristnika koji može stvoriti baze podataka, upravjati ulogama i zatraži lozinku
- `createuser -D -R korisničko_ime` : stvori koristnika bez mogućnosti stvaranja baza podataka ili upravljanja ulogama

## Links
- More information: https://www.postgresql.org/docs/current/app-createuser.html
