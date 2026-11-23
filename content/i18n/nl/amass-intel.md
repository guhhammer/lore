# amass intel

Verzamel open source informatie over een organisatie, zoals hoofddomeinen en ASN's.

## Usage
- `amass intel -addr 192.168.0.1-254` : vind hoofddomeinen in een IP-adresbereik ([addr])
- `amass intel -active -addr 192.168.0.1-254` : gebruik actieve verkenningsmethoden
- `amass intel -whois -d domeinnaam` : vind hoofddomeinen gerelateerd aan een domein
- `amass intel -org organisatienaam` : vind ASN's die bij een [org]anisatie horen
- `amass intel -asn asn` : vind hoofddomeinen die bij een bepaald ASN horen
- `amass intel -o uitvoer_bestand -whois -d domeinnaam` : sla de resultaten op in een tekstbestand
- `amass intel -list` : toon alle beschikbare databronnen
