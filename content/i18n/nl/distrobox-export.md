# distrobox-export

Exporteer app/service/binary van container naar host-besturingssysteem.

## Usage
- `distrobox-export -a pakket -ef "--foreground"` : exporteer een app van de container naar de host (het bureaublad-item/pictogram verschijnt in de applicatielijst van uw hostsysteem)
- `distrobox-export -b pad/naar/binary -ep pad/naar/binary_op_host` : exporteer een binary van de container naar de host
- `distrobox-export -b pad/naar/binary -ep pad/naar/export` : exporteer een binary van de container naar de host (bijv.`$HOME/.local/bin`)
- `distrobox-export --service pakket -ef "--allow-newer-config" -S` : exporteer een service van de container naar de host (`--sudo` zal de service uitvoeren als root in de container)
- `distrobox-export -a pakket -d` : verwijder een geëxporteerde applicatie

## Links
- More information: https://distrobox.it/usage/distrobox-export/
