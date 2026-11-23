# nmap

Netwerkverkenningstool en beveiligings-/poortscanner. Sommige functies (bijv. SYN-scan) worden alleen geactiveerd wanneer `nmap` wordt uitgevoerd met rootprivileges.

## Usage
- `nmap -v1|2|3 ip_of_hostnaam` : scan de top 1000 poorten van een externe host met verschillende verbositeitsniveaus
- `nmap -T5 -sn 192.168.0.0/24|ip_of_hostnaam1,ip_of_hostnaam2,...` : voer zeer agressief een ping-sweep uit over een heel subnet of individuele hosts
- `sudo nmap -A -iL pad/naar/bestand.txt` : schakel OS-detectie, versiedetectie, scriptscanning en traceroute in voor hosts uit een bestand
- `nmap -p poort1,poort2,... ip_of_host1,ip_of_host2,...` : scan een specifieke lijst van poorten (gebruik `-p-` voor alle poorten van 1 tot 65535)
- `nmap -sC -sV -oA top-1000-ports ip_of_host1,ip_of_host2,...` : voer service- en versiedetectie uit van de top 1000 poorten met standaard NSE-scripts, en schrijf resultaten (`-oA`) naar uitvoerbestanden
- `nmap --script "default and safe" ip_of_host1,ip_of_host2,...` : scan doelen voorzichtig met de `default and safe` NSE-scripts
- `nmap --script "http-*" ip_of_host1,ip_of_host2,... -p 80,443` : scan naar webservers die draaien op de standaard poorten 80 en 443 met alle beschikbare `http-*` NSE-scripts
- `sudo nmap -T0 -D decoy_ip1,decoy_ip2,... --source-port 53 -f --data-length 16 -Pn ip_of_host` : probeer IDS/IPS-detectie te omzeilen door gebruik te maken van een extreem trage scan (`-T0`), Decoy-bronadressen, fragmentatie van pakketten, willekeurige data en andere methoden

## Links
- More information: https://nmap.org/book/man.html
