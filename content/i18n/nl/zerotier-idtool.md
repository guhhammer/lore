# zerotier-idtool

Maak en manipuleer ZeroTier-identiteiten.

## Usage
- `zerotier-idtool generate` : genereer een nieuwe ZeroTier-identiteit en print het geheime deel naar `stdout`
- `zerotier-idtool generate pad/naar/identiteit.secret pad/naar/identiteit.public` : genereer een nieuwe ZeroTier-identiteit en sla het geheime en publieke deel op in bestanden
- `zerotier-idtool generate pad/naar/identiteit.secret pad/naar/identiteit.public vanity_voorvoegsel` : genereer een nieuwe ZeroTier-identiteit met een specifiek hexadecimaal vanity-voorvoegsel (kan lang duren)
- `zerotier-idtool getpublic pad/naar/identiteit.secret` : extraheer het publieke deel uit een geheime identiteit
- `zerotier-idtool sign pad/naar/identiteit.secret pad/naar/bestand` : onderteken een bestand met een geheime identiteit
- `zerotier-idtool verify pad/naar/identiteit.public pad/naar/bestand handtekening_hex` : verifieer een ondertekend bestand met een publieke identiteit en een hexadecimale handtekening
- `zerotier-idtool validate pad/naar/identiteit.public` : valideer lokaal de sleutel en proof-of-work van een identiteit
- `zerotier-idtool help` : toon de help

## Links
- More information: https://github.com/zerotier/ZeroTierOne/blob/dev/doc/zerotier-idtool.1.md
