# cupsd

Daemon de servidor para o servidor de impressão CUPS.

## Usage
- `cupsd` : inicia o `cupsd` em segundo plano, ou seja, como um daemon
- `cupsd -f` : inicia o `cupsd` em primeiro plano
- `cupsd -l` : inicia o `cupsd` sob demanda (usado comumente pelo `launchd` ou `systemd`)
- `cupsd -c caminho/para/cupsd.conf` : inicia o `cupsd` usando o arquivo de configuração [`c`]`upsd.conf`
- `cupsd -s caminho/para/arquivos-cups.conf` : inicia o `cupsd` usando os arquivos de configuração no arquivo `cups-file`[`s`]`.conf`
- `cupsd -t -c caminho/para/cupsd.conf` : testa o arquivo de configuração [`c`]`upsd.conf` por erros
- `cupsd -t -s caminho/para/arquivos-cups.conf` : testa os arquivos de configuração no arquivo `cups-file`[`s`]`.conf` por erros
- `cupsd -h` : mostra todas as opções disponíveis

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsd.html
