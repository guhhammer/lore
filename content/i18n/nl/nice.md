# nice

Voer een programma uit met een aangepaste planningsprioriteit (niceness). Niceness-waarden variëren van -20 (de hoogste prioriteit) tot 19 (de laagste). Opmerking: sommige moderne planners negeren niceness of beperken de effecten ervan binnen autogroepen.

## Usage
- `nice` : toon de huidige niceness-waarde
- `nice nice` : verhoog de huidige niceness-waarde met 10
- `nice -niceness_waarde commando` : start een programma met lagere prioriteit
- `sudo nice --niceness_waarde commando` : start een programma met verhoogde prioriteit
- `nice -n -20..19 commando` : definieer de prioriteit met een expliciete optie

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/nice-invocation.html
