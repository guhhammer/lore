# hlsq

Anzeige von HLS-Manifesten mit Farbhervorhebung und einfacher Filterung.

## Usage
- `curl --silent url | hlsq` : zeige ein HLS-Manifest von einer URL an
- `cat path/to/file.m3u8 | hlsq` : zeige ein HLS-Manifest aus einer Datei an
- `hlsq -watch -url url` : lade eine URL kontinuierlich neu und aktualisiere die Ausgabe
- `cat path/to/file.m3u8 | hlsq -query 'type = SUBTITLES'` : filtere eine Multivarianten-Playlist nach einem Attribut (Textwert)
- `cat path/to/file.m3u8 | hlsq -query 'bandwidth > 1000000'` : filtere eine Multivarianten-Playlist nach einem Attribut (Zahlenwert)

## Links
- More information: https://github.com/soldiermoth/hlsq/
