# kill

Envia um sinal para um processo, geralmente para finalizar o processo. Todos os sinais exceto pelo `SIGKILL` e `SIGSTOP` podem ser interceptados pelo processo para finalizar de forma limpa.

## Usage
- `kill id_do_processo` : finaliza um programa usando o sinal padrão `SIGTERM` (terminate)
- `kill -l` : lista todos os nomes de sinais disponíveis (para serem usados sem o prefixo `SIG`)
- `kill -1 id_do_processo` : finaliza um programa usando o sinal `SIGHUP`. Muitos daemons vão recarregar ao invés de finalizar
- `kill -2 id_do_processo` : finaliza um programa usando o sinal `SIGINT` (interrupt). Isto é tipicamente iniciado pelo usuário ao pressionar `<Ctrl c>`
- `kill -9 id_do_processo` : envia sinal para o sistema operacional para finalizar imediatamente o programa (que não tem chance de capturar o sinal)
- `kill -STOP id_do_processo` : envia sinal para o sistema operacional para pausar o programa até que um sinal `SIGCONT` ("continue") seja recebido
- `kill -USR1 -id_do_grupo` : envia um sinal `SIGUSR1` para todos os processos de um dado GID (group id)

## Links
- More information: https://manned.org/kill.1posix
