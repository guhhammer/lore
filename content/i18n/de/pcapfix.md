# pcapfix

Repariere beschädigte oder korrumpierte `pcap`- und `pcapng`-Dateien.

## Usage
- `pcapfix pfad/zu/datei.pcapng` : repariere eine `pcap`/`pcapng`-Datei (Hinweis: bei `pcap`-Dateien werden nur die ersten 262144 Bytes jedes Pakets gescannt)
- `pcapfix --deep-scan pfad/zu/datei.pcap` : repariere eine ganze `pcap`-Datei
- `pcapfix --outfile pfad/zu/Repariere.pcap pfad/zu/datei.pcap` : repariere eine `pcap`/`pcapng`-Datei und schreibe die Reparieree Datei an einen bestimmten Speicherort
- `pcapfix --pcapng pfad/zu/datei.pcapng` : behandle die zu reparierende Datei als `pcapng`-Datei, unabhängig von der automatischen Typenerkennung
- `pcapfix --verbose pfad/zu/datei.pcap` : repariere eine Datei und zeige den Reparaturprozess im Detail

## Links
- More information: https://f00l.de/pcapfix/
