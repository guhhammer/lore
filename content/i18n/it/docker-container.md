# docker container

Gestisci container Docker.

## Usage
- `docker ps` : elenca i container Docker attualmente in esecuzione
- `docker start nome_container1 nome_container2 ...` : avvia uno o più container fermi
- `docker kill nome_container1 nome_container2 ...` : termina uno o più container in esecuzione
- `docker stop nome_container1 nome_container2 ...` : ferma uno o più container in esecuzione
- `docker pause nome_container1 nome_container2 ...` : sospendi tutti i processi dentro uno o più container
- `docker container inspect nome_container1 nome_container2 ...` : mostra informazioni dettagliate su uno o più container
- `docker export nome_container` : esporta il filesystem di un container come archivio `.tar`
- `docker commit nome_container` : crea una nuova immagine dai cambiamenti di un container

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/
