# systemd-run

Executa programas em unidades temporárias de scope, service, ou unidades ativadas por path, socket ou timer.

## Usage
- `sudo systemd-run comando argumento1 argumento2 ...` : inicia um serviço temporário
- `systemd-run --user comando argumento1 argumento2 ...` : inicia um serviço temporário sob o gerenciador de serviços do usuário atual (sem privilégios)
- `sudo systemd-run -u nome --description texto comando argumento1 argumento2 ...` : inicia um serviço temporário com nome e descrição da unidade customizados
- `sudo systemd-run -r -E nome=valor comando argumento1 argumento2 ...` : inicia um serviço temporário que não será limpo após terminar com uma variável de ambiente customizada
- `sudo systemd-run --on-calendar=evento_do_calendário comando argumento1 argumento2 ...` : inicia um timer temporário que executa periodicamente seu serviço temporário (veja `man systemd.time` para o formato de eventos do calendário)
- `systemd-run -r --pty comando` : compartilha o terminal com o programa (permitindo entrada/saída interativas) e garante que os detalhes da execução permaneçam após a saída
- `systemd-run -p MemoryMax=memória_em_bytes -p CPUQuota=porcentagem_de_cpu% --wait comando` : define propriedades (ex.: CPUQuota, MemoryMax) do processo e aguarda sua finalização
- `comando1 | systemd-run -P comando2 | comando3` : usa o programa em uma pipeline do shell

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-run.html
