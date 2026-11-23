# naabu

Een snelle poortscanner geschreven in Go, gericht op betrouwbaarheid en eenvoud. Opmerking: sommige functies worden alleen geactiveerd wanneer `naabu` wordt uitgevoerd met rootprivileges, zoals SYN-scan.

## Usage
- `sudo naabu -host host` : voer een SYN-scan uit tegen de standaard (top 100) poorten van een externe host
- `naabu -il` : toon beschikbare netwerkinterfaces en het publieke IP-adres van de lokale host
- `naabu -p - -host host` : scan alle poorten van de externe host (CONNECT-scan zonder `sudo`)
- `naabu -tp 1000 -host host` : scan de top 1000 poorten van de externe host
- `naabu -p 80,443,u:53 -host host` : scan TCP-poorten 80, 443 en UDP-poort 53 van de externe host
- `naabu -p 80,443 -cdn -host host` : toon het CDN-type dat de externe host gebruikt, indien aanwezig
- `sudo naabu -v -host host -nmap-cli 'nmap -v -T5 -sC'` : voer `nmap` uit vanuit `naabu` voor extra functionaliteiten (`nmap` moet geïnstalleerd zijn)

## Links
- More information: https://docs.projectdiscovery.io/opensource/naabu/running
