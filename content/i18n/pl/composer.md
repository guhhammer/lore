# composer

Menadżer pakietów dla projektów PHP.

## Usage
- `composer init` : interaktywnie utwórz plik `composer.json`
- `composer require użytkownik/pakiet` : dodaj pakiet do zależności tego projektu, dodając wpis do `composer.json`
- `composer install` : zainstaluj wszystkie zależności z projektowego `composer.json` i utwórz `composer.lock`
- `composer remove użytkownik/pakiet` : odinstaluj pakiet z tego projektu, usuwając go jako zależność z `composer.json` i `composer.lock`
- `composer update` : zaktualizuj wszystkie pakiety z projektowego `composer.json` i zanotuj nową wersję w `composer.lock`
- `composer update --lock` : zaktualizuj tylko `composer.lock` po ręcznej aktualizacji `composer.json`
- `composer why-not użytkownik/pakiet` : dowiedz się więcej o powodach dlaczego zależność nie może zostać zainstalowana
- `composer self-update` : zaktualizuj narzędzie composer do najnowszej wersji

## Links
- More information: https://getcomposer.org/doc/03-cli.md
