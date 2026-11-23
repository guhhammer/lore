# setarch

Verander de gerapporteerde architectuur voor de uitvoering van een programma, voornamelijk gebruikt om aan te passen hoe programma's zich gedragen op basis van de systeemarchitectuur. Nuttig voor het testen van compatibiliteit of het draaien van oudere toepassingen.

## Usage
- `setarch i686 commando` : voer een commando uit alsof de machine-architectuur `i686` is (handig voor het draaien van 32-bit applicaties op een 64-bit kernel)
- `setarch x86_64 bash` : voer een shell uit met de `x86_64` architectuur
- `setarch linux32 -R commando` : schakel de willekeurigheid van de virtuele adresruimte uit
- `setarch --list` : toon ondersteunde architecturen
- `setarch -h` : toon de help

## Links
- More information: https://manned.org/setarch
