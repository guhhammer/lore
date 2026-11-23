# choco source

Gerenciar fontes para pacotes com Chocolatey.

## Usage
- `choco source list` : lista fontes atualmente disponíveis
- `choco source add -n nome -s url_da_fonte` : adiciona uma nova fonte de pacotes
- `choco source add -n nome -s url_da_fonte -u nome -p senha` : adiciona uma nova fonte de pacotes com credenciais
- `choco source add -n nome -s url_da_fonte --cert caminho/para/certificado` : adiciona uma nova fonte de pacotes com certificado do cliente
- `choco source enable -n nome` : habilita uma fonte de pacotes
- `choco source disable -n nome` : desabilita uma fonte de pacotes
- `choco source remove -n nome` : remove uma fonte de pacotes

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
