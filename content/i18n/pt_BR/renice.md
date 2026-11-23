# renice

Altera a prioridade/agradabilidade de agendamento de um ou mais processos em execução. Os valores de agradabilidade variam de -20 (mais favorável ao processo) a 19 (menos favorável ao processo).

## Usage
- `renice -n 3 -p pid` : altera a prioridade de um processo em execução
- `renice -n -4 -u nome_do_usuario` : altera a prioridade de todos os processos pertencentes a um usuário
- `renice -n 5 -g grupo_de_processos` : altera a prioridade de todos os processos que pertencem a um grupo de processos

## Links
- More information: https://manned.org/renice.1p
