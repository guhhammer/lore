# transmission-cli

Een lichtgewicht, command-line BitTorrent client. Opmerking: Dit commando is verouderd, gebruik in plaats daarvan `transmission-remote`.

## Usage
- `transmission-cli url|magnet|pad/naar/bestand` : download een specifieke torrent
- `transmission-cli -w pad/naar/download_map url|magnet|pad/naar/bestand` : download een torrent naar een specifieke map
- `transmission-cli --new pad/naar/bronbestand_of_map` : maak een torrentbestand van een specifiek bestand of map
- `transmission-cli -d 50 url|magnet|pad/naar/bestand` : specificeer de downloadsnelheidslimiet (in KB/s)
- `transmission-cli -u 50 url|magnet|pad/naar/bestand` : specificeer de uploadsnelheidslimiet (in KB/s)
- `transmission-cli -p poort_nummer url|magnet|pad/naar/bestand` : gebruik een specifieke poort voor verbindingen
- `transmission-cli -er url|magnet|pad/naar/bestand` : forceer versleuteling voor alle peer-verbindingen
- `transmission-cli -b blocklist_url|pad/naar/blocklist url|magnet|pad/naar/bestand` : gebruik een Bluetack-geformatteerde peer-blocklist

## Links
- More information: https://manned.org/transmission-cli
