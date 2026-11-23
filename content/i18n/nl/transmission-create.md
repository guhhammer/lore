# transmission-create

Maak BitTorrent `.torrent` bestanden.

## Usage
- `transmission-create -o pad/naar/voorbeeld.torrent -t aankondigings-url_van_tracker -s 2048 pad/naar/bestand_of_map` : maak een torrent met een specifieke stukgrootte (in KB)
- `transmission-create -p -o pad/naar/voorbeeld.torrent -t aankondigings-url_van_tracker -s 2048 pad/naar/bestand_of_map` : maak een privé torrent met een specifieke stukgrootte (in KB)
- `transmission-create -o pad/naar/voorbeeld.torrent -t tracker_url1 -c opmerking pad/naar/bestand_of_map` : maak een torrent met een opmerking
- `transmission-create -o pad/naar/voorbeeld.torrent -t tracker_url1 -t tracker_url2 pad/naar/bestand_of_map` : maak een torrent met meerdere trackers
- `transmission-create -h` : toon de help

## Links
- More information: https://manned.org/transmission-create
