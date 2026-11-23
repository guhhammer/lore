# pw-container

Esegue un programma in un nuovo contesto di sicurezza.

## Usage
- `pw-container` : crea un nuovo contesto di sicurezza e stampa il suo indirizzo socket su `stdout`
- `pw-container comando argomento1 argomento2 ...` : esegue un programma specifico in un nuovo contesto di sicurezza
- `pw-container -r nome_istanza_remota comando` : esegue un programma, collegandosi a un'istanza PipeWire remota specifica
- `pw-container -P '{"chiave": "valore"}' comando` : esegue un programma in un nuovo contesto con proprietà specifiche usando un oggetto JSON
- `pw-container -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-container_1.html
