# distrobox-export

Esporta app/servizio/binario dal container all'OS host.

## Usage
- `distrobox-export -a pacchetto -ef "--foreground"` : esporta un'app dal container all'host (la voce desktop/icona apparirà nell'elenco delle applicazioni del tuo sistema host)
- `distrobox-export -b percorso/del/binary -ep percorso/del/binary_on_host` : esporta un binario dal container all'host
- `distrobox-export -b percorso/del/binary -ep percorso/dell/export` : esporta un binario dal container all'host (ad es. `$HOME/.local/bin`)
- `distrobox-export --service pacchetto -ef "--allow-newer-config" -S` : esporta un servizio dal container all'host (`--sudo` eseguirà il servizio come root dentro il container)
- `distrobox-export -a pacchetto -d` : annulla l'esportazione/elimina un'applicazione esportata

## Links
- More information: https://distrobox.it/usage/distrobox-export/
