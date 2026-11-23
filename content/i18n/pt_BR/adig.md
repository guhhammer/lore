# adig

Imprime informações recebidas dos servidores do Sistema de Domínio de Nome(DNS).

## Usage
- `adig examplo.com` : exibe uma gravação A (padrão) do DNS por nome(s) de servidor(es)
- `adig -d examplo.com` : exibe uma saída de depuração extra
- `adig -s 1.2.3.4 examplo.com` : conecte-se a um servidor DNS específico
- `adig -T port examplo.com` : use uma porta TCP específica para se conectar ao servidor DNS
- `adig -U port examplo.com` : use uma porta UDP específica para se conectar ao servidor DNS

## Links
- More information: https://manned.org/adig
