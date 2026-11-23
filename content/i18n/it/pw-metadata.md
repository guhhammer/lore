# pw-metadata

Monitora, imposta ed elimina metadati sugli oggetti PipeWire.

## Usage
- `pw-metadata` : mostra i metadati nel nome `default`
- `pw-metadata -n settings 0` : mostra i metadati con ID 0 in `settings`
- `pw-metadata -l` : elenca tutti gli oggetti metadati disponibili
- `pw-metadata -m` : rimane in esecuzione e registra le modifiche ai metadati
- `pw-metadata -d` : elimina tutti i metadati
- `pw-metadata -n settings 0 log.level 1` : imposta `log.level` a 1 in `settings`
- `pw-metadata -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-metadata_1.html
