# rg

Ripgrep, een recursieve regel-georiënteerde zoektool. Wil een sneller alternatief zijn dan `grep`.

## Usage
- `rg patroon` : zoek recursief in de huidige map naar een patroon (`regex`)
- `rg patroon pad/naar/bestand_of_map` : zoek recursief naar een patroon in een bestand of map
- `rg -F -- string` : zoek naar een letterlijk stringpatroon
- `rg -. --no-ignore patroon` : voeg verborgen bestanden en onderdelen van de `.gitignore` toe
- `rg patroon -g 'bestandsnaam_glob_patroon'` : zoek in bestanden die overeenkomen met een `glob` (bijv. `README.*`, gebruik `!bestandsnaam_patroon` om in plaats daarvan uit te sluiten) naar een patroon
- `rg --files | rg --passthru patroon` : toon recursief de bestandsnamen in de huidige map en markeer degene die overeenkomen met een patroon
- `rg -l patroon` : toon alleen overeenkomende bestanden (handig bij het doorsturen naar andere commando's)
- `rg -v patroon` : toon regels die niet overeenkomen met het patroon

## Links
- More information: https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md
