# docker inspect

Mostra informazioni a basso livello di oggetti Docker.

## Usage
- `docker inspect nome_container|nome_immagine|id` : mostra informazioni su un container, immagine o volume usando un nome o un identificativo (ID)
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' nome_container` : mostra l'indirizzo IP di un container
- `docker inspect -f '\{\{.LogPath\}\}' nome_container` : mostra il percorso dei file di log di un container
- `docker inspect -f '\{\{.Config.Image\}\}' nome_container` : mostra il nome dell'immagine di un container
- `docker inspect -f '\{\{json .Config\}\}' nome_container` : mostra le informazioni di configurazione in formato JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' nome_container` : mostra il binding di tutte le porte
- `docker inspect` : mostra l'aiuto

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
