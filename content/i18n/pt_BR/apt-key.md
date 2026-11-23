# apt-key

Gerenciador de chaves utilizado pelo gerenciador de pacotes APT nas distribuições baseadas em Debian.

## Usage
- `apt-key list` : exibe as chaves confiáveis
- `apt-key add arquivo_da_chave_publica.asc` : adiciona uma chave na lista de chaves confiáveis
- `apt-key del key_id` : remove uma chave da lista de chaves confiáveis
- `wget -qO - https://host.tld/arquivo.key | apt-key add -` : adiciona uma chave remota na lista de chaves confiáveis
- `apt-key adv --keyserver pgp.mit.edu --recv key_id` : adiciona uma chave, de um servidor de chaves, na lista de chaves confiáveis

## Links
- More information: https://manned.org/apt-key
