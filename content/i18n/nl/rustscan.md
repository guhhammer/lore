# rustscan

Moderne poortscanner geschreven in Rust.

## Usage
- `rustscan -a ip_of_hostnaam1,ip_of_hostnaam2,...` : scan alle poorten van één of meerdere door komma's gescheiden adressen met de standaardwaarden
- `rustscan --top -a adres` : scan de top 1000 poorten met service- en versiedetectie
- `rustscan -p poort1,poort2,... -a adres` : scan een specifieke lijst van poorten
- `rustscan -r start-einde -a adres` : scan een specifiek bereik van poorten
- `rustscan -a adres -- -O -sC` : roep `nmap`-functionaliteiten aan (Nmap's OS-detectie en standaardscripts)
- `rustscan -b batch_grootte -t timeout -a adres` : scan met een aangepaste batchgrootte (standaard: 4500) en timeout (standaard: 1500ms)
- `rustscan --scan-order serial|random -a adres` : scan met een specifieke poortvolgorde
- `rustscan -g -a adres` : scan in greppable modus (alleen output van de poorten, geen `nmap`)

## Links
- More information: https://github.com/bee-san/RustScan/wiki
