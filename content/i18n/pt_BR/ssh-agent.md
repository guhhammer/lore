# ssh-agent

Iniciar um processo do Agente SSH. Um Agente SSH mantém as chaves SSH descriptografadas na memória até serem removidas ou o processo ser encerrado.

## Usage
- `eval $(ssh-agent)` : inicia um Agente SSH para o shell atual
- `ssh-agent -k` : encerra o Agente em execução atualmente

## Links
- More information: https://man.openbsd.org/ssh-agent
