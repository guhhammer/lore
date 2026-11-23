# sort

Sorteer regels van tekstbestanden.

## Usage
- `sort pad/naar/bestand` : sorteer een bestand in oplopende volgorde
- `sort -r pad/naar/bestand` : sorteer een bestand in aflopende volgorde
- `sort -f pad/naar/bestand` : sorteer een bestand op een niet-hoofdlettergevoelige manier
- `sort -n pad/naar/bestand` : sorteer een bestand met numerieke in plaats van alfabetische volgorde
- `sort -t : -k 3n /etc/passwd` : sorteer `/etc/passwd` numeriek vanaf het 3e veld van elke regel, gebruikmakend van `:` als veldscheidingsteken
- `sort -t : -k 3,3n -k 4,4g /etc/passwd` : sorteer `/etc/passwd` numeriek op het 3e veld, en wanneer items in het 3e veld gelijk zijn, sorteer op het 4e veld met getallen en exponenten
- `sort -u pad/naar/bestand` : sorteer een bestand waarbij alleen unieke regels worden behouden
- `sort -o pad/naar/uitvoer_bestand pad/naar/invoer_bestand` : sorteer een bestand en schrijf de uitvoer naar het opgegeven uitvoerbestand (kan worden gebruikt om een bestand in-place te sorteren)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/sort-invocation.html
