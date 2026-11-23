# bgpgrep

Filtert und gibt BGP-Daten in einem MRT-Dump aus. Kann mit gzip, bzip2 und xz komprimierte Dateien lesen.

## Usage
- `bgpgrep master6.mrt` : gib alle Routen aus
- `bgpgrep master4.mrt -peer 64498` : gib alle von einem bestimmten Peer empfangenen Routen aus, bestimmt durch die AS-Nummer des Peers
- `bgpgrep master4.mrt.bz2 -peer 2001:db8:dead:cafe:acd::19e` : gib alle von einem bestimmten Peer empfangenen Routen aus, bestimmt durch die IP-Adresse des Peers
- `bgpgrep master6.mrt.bz2 -aspath '64498 64510'` : gib alle Routen aus, die bestimmte ASNs in ihrem AS-Pfad haben
- `bgpgrep master6.mrt.bz2 -supernet '2001:db8:dead:cafe:aef::5'` : gib Routen aus, die zu einer bestimmten Adresse führen
- `bgpgrep master4.mrt -communities \( '64497:*' \)` : gib alle Routen aus, die Communities von einem bestimmten AS haben

## Links
- More information: https://codeberg.org/1414codeforge/ubgpsuite
