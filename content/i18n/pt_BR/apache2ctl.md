# apache2ctl

Interface de controle do servidor web HTTP Apache. Este comando está disponível nas distribuições baseadas em Debian, para as baseadas em RHEL veja `httpd`.

## Usage
- `sudo apache2ctl start` : inicia o Apache. Caso ele já esteja em execução, uma mensagem será apresentada
- `sudo apache2ctl stop` : encerra o Apache
- `sudo apache2ctl restart` : reinicia o Apache
- `sudo apache2ctl -t` : verifica se o arquivo de configuração está correto sintaticamente
- `sudo apache2ctl -M` : lista os módulos carregados

## Links
- More information: https://manned.org/apache2ctl
