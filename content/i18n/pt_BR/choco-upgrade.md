# choco upgrade

Atualizar um ou mais pacotes com Chocolatey.

## Usage
- `choco upgrade pacote(s)` : atualiza um ou mais pacotes separados por espaço
- `choco upgrade pacote --version versão` : atualiza para uma versão específica de um pacote
- `choco upgrade all` : atualiza todos pacotes
- `choco upgrade all --except "pacote(s)"` : atualiza todos os pacotes, exceto os especificados separados por virgula
- `choco upgrade pacote -y` : confirma todos os prompts automaticamente
- `choco upgrade pacote -s url_do_pacote|apelido` : especifique uma fonte personalizada para receber pacotes
- `choco upgrade pacote -u usuário -p senha` : fornece um nome e uma senha para autenticação

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
