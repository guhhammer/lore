# docker container exec

Esegui un comando su un Docker container in esecuzione.

## Usage
- `docker exec -it nome_container /bin/bash` : avvia una shell interattiva all'interno di un container in esecuzione
- `docker exec -d nome_container comando` : esegui un commando in background ("detached") su un container in esecuzione
- `docker exec -it -w percorso/della/directory nome_container comando` : seleziona la directory di lavoro in cui eseguire un dato comando
- `docker exec -i -d nome_container comando` : esegui un comando in background su un container esistente, mantenendo aperto `stdin`
- `docker exec -it -e nome_variabile=valore nome_container /bin/bash` : imposta una variabile d'ambiente in una sessione Bash in esecuzione
- `docker exec -u utente nome_container comando` : specifica l'utente da usare per eseguire un comando

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
