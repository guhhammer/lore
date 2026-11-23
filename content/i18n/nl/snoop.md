# snoop

Netwerkpakketsniffer. SunOS equivalent van `tcpdump`.

## Usage
- `snoop -d e1000g0` : leg de pakketten van een specifieke netwerkinterface vast
- `snoop -o pad/naar/bestand` : sla de pakketten op in een bestand, in plaats van ze weer te geven
- `snoop -V -i pad/naar/bestand` : toon de verbose protocollaag-samenvatting van de pakketten in een bestand
- `snoop to port poort from host hostnaam` : leg netwerkpakketten vast die van een bepaalde host komen en naar een gegeven poort gaan
- `snoop -x0 -p4 ip_adres_1 ip_adres_2` : leg een hex-dump vast van netwerkpakketten die uitgewisseld zijn tussen twee IP-adressen en toon deze

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
