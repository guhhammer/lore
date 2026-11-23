# argos-translate

Een open-source offline vertaalbibliotheek en CLI-tool geschreven in Python.

## Usage
- `argospm install translate-es_en` : installeer vertaalparen voor vertaling van Spaans naar Engels
- `argos-translate --from-lang es --to-lang en un texto corto` : vertaal een tekst van het Spaans (`es`) naar het Engels (`en`) (Opmerking: alleen taalcodes van twee letters worden ondersteund)
- `cat pad/naar/bestand.txt | argos-translate --from-lang en --to-lang hi` : vertaal een tekstbestand van het Engels naar het Hindi
- `argospm list` : toon alle geïnstalleerde vertaalparen
- `argospm search --from-lang en` : toon vertaalparen vanuit het Engels die beschikbaar zijn om te installeren
- `argospm update` : update geïnstalleerde taalpakketparen
- `argos-translate --from-lang ar --to-lang ru صورة تساوي أكثر من ألف كلمة` : vertaal van `ar` naar `ru` (Opmerking: dit vereist dat de vertaalparen `translate-ar_en` en `translate-en_ru` geïnstalleerd zijn)

## Links
- More information: https://argos-translate.readthedocs.io/en/latest/source/cli.html
