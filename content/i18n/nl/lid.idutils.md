# lid.idutils

Bevraag een ID-database op tokens die overeenkomen met een patroon. Opmerking: er moet eerst een ID-database gebouwd worden met `mkid`.

## Usage
- `lid` : toon alle tokens en hun bestandslocaties in de ID-database
- `lid token` : zoek bestanden die een specifiek token bevatten
- `lid -i token` : zoek tokens die overeenkomen met een patroon, ongeacht hoofdlettergebruik
- `lid -r "patroon"` : zoek tokens die overeenkomen met een uitgebreide `regex`
- `lid -R grep token` : toon overeenkomende regels in grep-stijl formaat
- `lid -F 1` : zoek tokens die slechts één keer voorkomen (handig voor het vinden van ongebruikte definities)

## Links
- More information: https://www.gnu.org/software/idutils/manual/idutils.html#lid-invocation
