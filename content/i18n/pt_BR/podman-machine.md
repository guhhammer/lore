# podman machine

Criar e gerenciar máquinas virtuais executando o Podman. Incluído com a versão 4 ou superior do Podman.

## Usage
- `podman machine ls` : lista as máquinas existentes
- `podman machine init` : cria uma nova máquina padrão
- `podman machine init nome` : cria uma nova máquina com um nome específico
- `podman machine init --cpus 4 --memory 4096 --disk-size 50` : cria uma nova máquina com recursos diferentes
- `podman machine start|stop nome` : inicia ou para uma máquina
- `podman machine ssh nome` : conecta-se a uma máquina em execução via SSH
- `podman machine inspect nome` : inspeciona informações sobre uma máquina

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-machine.1.html
