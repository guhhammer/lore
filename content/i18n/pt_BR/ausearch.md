# ausearch

Consulta eventos nos logs de auditoria do Linux. Parte do pacote `audit`.

## Usage
- `sudo ausearch -m avc` : pesquisa todos os eventos de negação AVC do SELinux
- `sudo ausearch -c httpd` : pesquisa eventos relacionados a um executável específico
- `sudo ausearch -ui 1000` : pesquisa eventos de um usuário específico
- `sudo ausearch -ts recent` : pesquisa eventos nos últimos 10 minutos
- `sudo ausearch -m user_login -sv no` : pesquisa tentativas de login com falha
- `sudo ausearch -f caminho/para/arquivo` : pesquisa eventos relacionados a um arquivo específico
- `sudo ausearch -m avc --raw` : exibe os resultados em formato bruto para processamento posterior

## Links
- More information: https://manned.org/ausearch
