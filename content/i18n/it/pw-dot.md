# pw-dot

Crea file `.dot` del grafico di PipeWire.

## Usage
- `pw-dot` : genera un grafico nel file `pw.dot`
- `pw-dot -j percorso/del/file.json` : legge oggetti da un file JSON `pw-dump`
- `pw-dot -o percorso/del/file.dot -a` : specifica un file di output, mostrando tutti i tipi di oggetti
- `pw-dot -o - -d` : stampa il grafico `.dot` su `stdout`, mostrando tutte le proprietà degli oggetti
- `pw-dot -r nome_remoto -s` : genera un grafico da un'istanza remota, mostrando solo gli oggetti collegati
- `pw-dot -L` : dispone il grafico da sinistra a destra, invece del predefinito dall'alto verso il basso
- `pw-dot -9` : dispone il grafico usando angoli di 90 gradi nei bordi
- `pw-dot -h` : mostra l'aiuto

## Links
- More information: https://docs.pipewire.org/page_man_pw-dot_1.html
