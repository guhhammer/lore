# egrep

Zoek naar patronen in bestanden met behulp van uitgebreide `regex`en. Opmerking: dit commando is een alias van `grep --extended-regexp`.

## Usage
- `egrep 'a+' pad/naar/bestand` : zoek naar één of meer herhaalde tekens
- `egrep 'a?' pad/naar/bestand` : zoek naar nul of één keer een teken (optionele overeenkomst)
- `egrep 'a{10}' pad/naar/bestand` : zoek naar 10 herhalingen van een teken
- `egrep 'a{3,7}' pad/naar/bestand` : zoek naar 3 tot 7 herhalingen van een karakter
- `egrep 'cat|dog|mouse' pad/naar/bestand` : zoek naar één van de vermelde opties
- `egrep 'c(a|o|u)p' pad/naar/bestand` : zoek naar één van de vermelde opties binnen een groter patroon
- `egrep '(aeiou)+' pad/naar/bestand` : zoek naar een groep tekens die één of meerdere keren herhaald worden
- `egrep [[:alnum:|:alpha:|:space:|...]] pad/naar/bestand` : zoek met behulp van standaard tekenklassen (meer informatie: <https://www.regular-expressions.info/posixbrackets.html>)

## Links
- More information: https://manned.org/egrep
