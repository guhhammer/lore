# docker

Gestisci container ed immagini Docker. Alcuni comandi aggiuntivi, come `run`, hanno la propria documentazione.

## Usage
- `docker ps -a` : elenca tutti i container Docker (in esecuzione e arrestati)
- `docker run --name nome_container immagine` : avvia un container da una immagine, con un nome personalizzato
- `docker start|stop nome_container` : avvia o arresta un container esistente
- `docker pull image` : scarica (pull) un'immagine dal Docker registry
- `docker exec -it nome_container sh` : avvia una shell all'interno di un container in esecuzione
- `docker rm nome_container` : rimuovi un container arrestato
- `docker logs -f nome_container` : ottieni e visualizza i log di un container

## Links
- More information: https://docs.docker.com/reference/cli/docker/
