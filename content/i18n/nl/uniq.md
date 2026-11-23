# uniq

Geef de unieke regels uit een invoer of bestand weer. Omdat het geen herhaalde regels detecteert tenzij ze naast elkaar staan, moeten we ze eerst sorteren.

## Usage
- `sort pad/naar/bestand | uniq` : toon elke regel één keer
- `sort pad/naar/bestand | uniq -u` : toon alleen unieke regels
- `sort pad/naar/bestand | uniq -d` : toon alleen dubbele regels
- `sort pad/naar/bestand | uniq -i` : toon elke regel één keer, hoofdletterongevoelig
- `sort pad/naar/bestand | uniq -c` : toon het aantal voorkomens van elke regel samen met die regel
- `sort pad/naar/bestand | uniq -c | sort -nr` : toon het aantal voorkomens van elke regel, gesorteerd op meest frequent
- `sort pad/naar/bestand | uniq -w 10` : vergelijk alleen de eerste 10 tekens van elke regel op uniekheid
- `sort pad/naar/bestand | uniq -s 5` : vergelijk tekst na de eerste 5 tekens van elke regel op uniekheid

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html
