# asr

Ripristina (copia) un'immagine disco dentro a un volume. Il nome del comando sta per Apple Software Restore (software di ripristino Apple).

## Usage
- `sudo asr restore --source nome_immagine.dmg --target percorso/del/volume` : ripristina un'immagine disco su un volume specifico
- `sudo asr restore --source nome_immagine.dmg --target percorso/del/volume --erase` : distruggi il volume specifico prima di ripristinare
- `sudo asr restore --source nome_immagine.dmg --target percorso/del/volume --noverify` : salta la verifica dopo il ripristino
- `sudo asr restore --source percorso/del/volume --target percorso/del/volume/clonato` : clona i volumi senza utilizzare un'immagine disco intermedia

## Links
- More information: https://keith.github.io/xcode-man-pages/asr.8.html
