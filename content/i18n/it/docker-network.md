# docker network

Crea e gestisci reti docker.

## Usage
- `docker network ls` : elenca le reti disponibili configurate sul Docker daemon
- `docker network create -d nome_del_driver nome_rete` : crea una rete definita da un utente
- `docker network inspect nome_rete_1 nome_rete_2` : mostra informazioni dettagliate su una lista di reti (separata da spazi)
- `docker network connect nome_rete nome_container|id` : connetti un container ad una rete usando il suo nome o ID
- `docker network disconnect nome_rete nome_container|id` : disconnetti un container da una rete
- `docker network prune` : elimina le reti inutilizzate (non referenziate da alcun container)
- `docker network rm nome_rete_1 nome_rete_2 ...` : elimina una lista di reti (separata da spazi) inutilizzate

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
