# dpkg-query

Mostra informações dos pacotes instalados.

## Usage
- `dpkg-query -l` : exibe todos os pacotes instalados
- `dpkg-query -l 'libc6*'` : exibe os pacotes instalados correspondentes ao critério de busca
- `dpkg-query -L libc6` : exibe todos os arquivos instalados por um pacote
- `dpkg-query -s libc6` : exibe informações sobre um pacote
- `dpkg-query -S /etc/ld.so.conf.d` : busca por pacotes que contenham arquivos que correspondam ao padrão

## Links
- More information: https://manned.org/dpkg-query
