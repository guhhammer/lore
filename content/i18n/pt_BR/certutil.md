# certutil

Gerencie chaves e certificados em bancos de dados e tokens NSS.

## Usage
- `certutil -N -d .` : cria um novo banco de dados de certificados
- `certutil -L -d .` : lista todos os certificados em um banco de dados
- `certutil -K -d . -f caminho/para/arquivo_de_senha.txt` : lista todas as chaves privadas em um banco de dados
- `certutil -A -n "certificado_do_servidor" -t ",," -i caminho/para/arquivo.crt -d .` : importa o certificado assinado para o banco de dados dos solicitantes
- `certutil -S -f caminho/para/arquivo_de_senha.txt -d . -t ",," -c "certificado_do_servidor" -n "nome_do_servidor" -g 2048 -s "CN=nome_comum,O=organização"` : adiciona nomes de assunto a um determinado certificado

## Links
- More information: https://manned.org/certutil
