# masscan

Een zeer snelle netwerkscanner. Werkt het beste met verhoogde privileges. Voor hulp met Nmap-compatibiliteit, voer `masscan --nmap` uit.

## Usage
- `masscan ip_adres|netwerk_prefix -p 80` : scan een IP of netwerksubnet op poort 80
- `masscan 10.0.0.0/16 --top-ports 100 --rate 100000` : scan een klasse B-subnet op de top 100 poorten met 100.000 pakketten per seconde
- `masscan 10.0.0.0/16 --top-ports 100 --excludefile pad/naar/bestand` : scan een klasse B-subnet en vermijd bereiken uit een specifiek uitsluitingsbestand
- `masscan 10.0.0.0/16 -p 22,80 --banners --rate 100000` : scan een klasse B-subnet met Nmap-achtige versiedetectie (banner grabbing)
- `masscan 0.0.0.0/0 -p 80,443 --rate 10000000` : scan het internet op webservers die draaien op poort 80 en 443
- `masscan 0.0.0.0/0 -p U:53 --rate 10000000` : scan het internet op DNS-servers die draaien op UDP-poort 53
- `masscan 0.0.0.0/0 -p 0-65535 --output-format binary|grepable|json|list|xml --output-filename pad/naar/bestand` : scan het internet op een specifiek poortbereik en exporteer naar een bestand
- `masscan --readscan pad/naar/bestand` : lees binaire scanresultaten uit een bestand en geef ze weer op `stdout`

## Links
- More information: https://manned.org/masscan
