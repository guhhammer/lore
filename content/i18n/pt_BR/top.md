# top

Mostra informações, em tempo real, sobre os processos em execução.

## Usage
- `top` : inicia o `top`
- `top -i` : exibe apenas os processos ativos
- `top -u usuario` : exibe os processos de um usuário específico
- `top -o nome_do_campo` : ordena os processos por campo
- `top -Hp id_do_processo` : mostra todas as threads de um dado processo
- `top -p $(pgrep -d ',' nome_do_processo)` : mostra apenas processos com determinados PID(s), informados em uma lista separada por vírgulas (Normalmente você não saberá os PIDs de cabeça. Este exemplo pega os PIDs a partir do nome de um processo)
- `<?>` : mostra ajuda sobre comandos interativos

## Links
- More information: https://manned.org/top
