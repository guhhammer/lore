# docker system

Gerenciar dados do Docker e exibir informações do sistema em todo o sistema.

## Usage
- `docker system` : mostra ajuda
- `docker system df` : mostra o uso de disco do Docker
- `docker system df -v` : mostra informações detalhadas sobre o uso de disco
- `docker system prune` : remove dados não utilizados
- `docker system prune --filter "until=horashminutosm"` : remove dados não utilizados criados há mais de um período específico no passado
- `docker system events` : exibe eventos em tempo real do daemon do Docker
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : exibe eventos em tempo real de contêineres transmitidos como JSON Lines válidos
- `docker system info` : exibe informações em todo o sistema

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
