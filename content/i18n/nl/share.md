# share

Maak lokale bron/bestandssysteem beschikbaar voor mounten door systemen op afstand.

## Usage
- `share` : toon alle momenteel gedeelde bestandssystemen
- `share -F nfs -o rw /pad/naar/map` : deel een map met lees/schrijftoegang
- `share -F nfs -o ro /pad/naar/map` : deel een map met alleen-lezen toegang
- `share -F nfs -o rw,root=hostnaam /pad/naar/map` : deel een map met specifieke opties (bijvoorbeeld root-toegang toestaan vanaf een bepaalde host)
- `echo "share -F nfs -o rw /pad/naar/map" >>/etc/dfs/dfstab` : maak delen persistent door vermeldingen toe te voegen aan `/etc/dfs/dfstab`

## Links
- More information: https://docs.oracle.com/cd/E36784_01/html/E36825/gntjt.html
