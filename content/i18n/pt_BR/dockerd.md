# dockerd

Um processo persistente para iniciar e gerenciar contêineres Docker.

## Usage
- `dockerd` : executa o daemon do Docker
- `dockerd -H unix://caminho/para/tmp.sock -H tcp://ip_address` : executa o daemon do Docker e configurá-lo para escutar em sockets específicos (UNIX e TCP)
- `dockerd -p caminho/para/arquivo_pid` : executa com um arquivo PID específico para o daemon
- `dockerd -D` : executa no modo de depuração
- `dockerd -l debug|info|warn|error|fatal` : executa e define um nível de log específico

## Links
- More information: https://docs.docker.com/reference/cli/dockerd/
