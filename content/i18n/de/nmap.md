# nmap

Netzwerk-Erkundungs-Werkzeug und Security / Port Scanner. Manche Funktionen können nur benutzt werden, wenn Nmap mit Root Rechten ausgeführt wird.

## Usage
- `nmap -O ip_oder_hostname` : überprüfe ob eine IP-Adresse online ist und versuche, das Betriebssystem herauszufinden
- `sudo nmap -sn ip_oder_hostname optional_noch_eine_addresse` : überprüfe nur ob die angegebenen Hosts online sind (Ping Scan) und ihre Domain-Namen
- `nmap -A addresse_oder_addressen` : scanne zusätzlich mit Skripten, Service-Erkennung, Betriebssystem-Fingerprinting und Traceroute
- `nmap -p port1,port2,...,portN addresse_oder_addressen` : scanne eine spezifische Liste an Ports (benutze '-p-' für alle Ports von 1 bis 65535)
- `nmap -sC -sV -oN ergebnis.txt addresse_oder_addressen` : führe Dienst- und Versions-Erkennung auf den top 1000 Ports mit den Standard NSE Skripten aus; und schreibe das Ergebnis ('-oN') in der Ausgabe Datei
- `nmap --script "default and safe" addresse_oder_addressen` : scanne Ziel(e) vorsichtig mit 'default and safe' NSE Scripts
- `nmap --script "http-*" addresse_oder_addressen -p 80,443` : scanne einen Web-Server, der auf den Standard Ports 80 und 443 läuft, mit allen verfügbaren 'http-*' NSE Skripten
- `nmap -T0 -D köder1_ipaddresse,köder2_ipaddresse,...,köderN_ipaddresse addresse_oder_addressen` : führe einen sehr langsamen verborgenen Scan ('-T0') aus um die Entdeckung von IDS/IPS zu umgehen und benutze Köder IP-Adressen ('-D')

## Links
- More information: https://nmap.org/book/man.html
