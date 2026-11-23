# docker swarm

Uma ferramenta de orquestração de contêineres.

## Usage
- `docker swarm init` : inicializa um cluster do Swarm
- `docker swarm join-token worker|manager` : exibe o token para ingressar como gerenciador ou trabalhador
- `docker swarm join --token token url_do_nó_gerenciador:2377` : ingressa um novo nó ao cluster
- `docker swarm leave` : remove um trabalhador do Swarm (executado dentro do nó trabalhador)
- `docker swarm ca` : exibe o certificado CA atual no formato PEM
- `docker swarm ca --rotate` : rotaciona o certificado CA atual e exibe o novo certificado
- `docker swarm update --cert-expiry horashminutosmsegundoss` : altera o período de validade dos certificados dos nós

## Links
- More information: https://docs.docker.com/engine/swarm/
