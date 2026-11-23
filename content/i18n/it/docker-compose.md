# docker compose

Esegui e gestisci applicazioni Docker composte da più container.

## Usage
- `docker compose ps` : elenca i container in esecuzione
- `docker compose up -d` : crea ed avvia tutti i container in background utilizzando il file `docker-compose.yml` nella directory corrente
- `docker compose up --build` : avvia tutti i container, buildandoli di nuovo se necessario
- `docker compose -p nome_di_progetto -f percorso/del/file up` : avvia tutti i contenitori specificando un nome di progetto e utilizzando un file compose alternativo
- `docker compose stop` : ferma tutti i container in esecuzione
- `docker compose down --rmi all -v` : ferma e rimuovi tutti i container, reti, immagini e volumi
- `docker compose logs -f` : segui i log di tutti i container
- `docker compose logs -f nome_container` : segui i log di un container specifico

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
