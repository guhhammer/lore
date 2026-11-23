# minecraft

Esegui un server di Minecraft senza interfaccia grafica.

## Usage
- `java -jar percorso/del/server.jar --nogui` : avvia un server Minecraft e genera un mondo se non esiste
- `java -Xms1024M -Xmx2048M -jar percorso/del/server.jar --nogui` : imposta il quantitativo minimo e massimo di memoria che il server può avere (Nota: Impostare gli stessi valori previene lag causati da heap scaling)
- `java -jar percorso/del/server.jar` : avvia un server con una GUI
- `stop` : [Interattivo] Spegne il server

## Links
- More information: https://minecraft.wiki/w/Tutorial:Setting_up_a_Java_Edition_server
