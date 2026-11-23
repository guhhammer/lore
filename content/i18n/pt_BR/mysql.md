# mysql

A ferramenta de linha de comando do MySQL.

## Usage
- `mysql nome_do_banco_de_dados` : conecta a um banco de dados
- `mysql -u usuário --password nome_do_banco_de_dados` : conecta a um banco de dados (será solicitada a senha de acesso do usuário)
- `mysql -h endereco_do_banco_de_dados nome_do_banco_de_dados` : conecta a um banco de dados disponível em um endereço específico
- `mysql --socket caminho/para/socket.sock` : conecta a um banco de dados utilizando um socket Unix
- `mysql -e "source nome_do_arquivo.sql" nome_do_banco_de_dados` : executa todos os comandos de um arquivo SQL em um banco de dados

## Links
- More information: https://manned.org/mysql
