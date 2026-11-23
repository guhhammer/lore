# pw-dump

Scarica lo stato corrente di PipeWire come JSON, incluse le informazioni su nodi, dispositivi, moduli, porte e altri oggetti.

## Usage
- `pw-dump` : stampa una rappresentazione JSON dello stato corrente dell'istanza PipeWire predefinita
- `pw-dump id_oggetto` : stampa una rappresentazione JSON di un oggetto
- `pw-dump -m` : scarica lo stato corrente monitorando i cambiamenti, stampandolo nuovamente
- `pw-dump -r nome_remoto >percorso/file_dump.json` : scarica lo stato corrente di un'istanza remota in un file
- `pw-dump -C never|always|auto` : imposta una configurazione colore
- `pw-dump -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-dump_1.html
