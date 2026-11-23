# hping3

Geavanceerd pinghulpprogramma dat protocollen ondersteunt zoals TCP, UDP en raw IP. Dit kan het beste uitgevoerd worden met extra privileges.

## Usage
- `hping3 -1 -c 4 ip_of_hostnaam` : ping een bestemming met 4 ICMP ping aanvragen
- `hping3 -2 -p 80 -S ip_of_hostnaam` : ping een IP adres over UDP op poort 80
- `hping3 -V -S -p 80 -s 5090 ip_of_hostnaam` : scan TCP poort 80, maar scan vanaf de specifieke lokale bronpoort 5090
- `hping3 -T -V -S -p 80 ip_of_hostnaam` : traceroute met behulp van een TCP scan naar een specifieke bestemmingspoort
- `hping3 -8 80,3000,9000 -S ip_of_hostnaam` : scan een set van TCP poorten op een specifiek IP adres
- `hping3 -c 2 -V -p 80 -A ip_of_hostnaam` : voer een TCP ACK scan uit om te checken of een gegeven host nog leeft
- `hping3 --flood -p 80 -S ip_of_hostnaam` : voer een charge test uit op poort 80

## Links
- More information: https://manned.org/hping3
