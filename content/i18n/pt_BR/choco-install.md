# choco install

Instalar um pacote ou mais com Chocolatey.

## Usage
- `choco install pacote(s)` : instala um ou mais pacotes separado por espaço
- `choco install caminho/para/pacotes.config` : instala pacotes a partir de um aquivo de configuração personalizado
- `choco install caminho/para/arquivo` : instala um arquivo específico `.nuspec` ou `.nupkg`
- `choco install pacote --version versão` : instala uma versão específica de um pacote
- `choco install pacote --allow-multiple` : permite a instalação de múltiplas versões de um pacote
- `choco install pacote -y` : confirma todos prompts automaticamente
- `choco install pacote -s url_do_pacote|apelido` : especifica uma fonte personalizada para receber pacotes
- `choco install pacote -u usuario -p senha` : fornece um nome e uma senha para autenticação

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
