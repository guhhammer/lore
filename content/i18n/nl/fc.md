# fc

Open de recente commando's voor bewerking en voer ze vervolgens uit.

## Usage
- `fc` : open het laatste commando in de standaard systeemeditor en voer het uit na het aanpassen
- `fc -e 'emacs'` : specificeer een editor om mee te openen
- `fc -l` : toon recente commando's uit de geschiedenis
- `fc -l -r` : toon recente commando's in omgekeerde volgorde
- `fc nummer` : pas een commando uit de geschiedenis aan en voer het uit
- `fc '416' '420'` : pas commando's in een gegeven interval aan en voer ze uit
- `fc -s string1=string2` : voer het vorige commando onmiddellijk uit zonder een editor te openen en vervang alle voorkomens van een string door een andere string
- `fc --help` : toon de help

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-fc
