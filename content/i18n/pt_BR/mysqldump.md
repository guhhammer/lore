# mysqldump

Realizar e restaurar backups no MySQL.

## Usage
- `mysqldump -u usuário --password nome_do_banco_de_dados -r arquivo_de_saida.sql` : cria o backup do banco de dados em arquivo de saída (será solicitada a senha de acesso do usuário)
- `mysql -u usuário --password -e "source arquivo_de_backup.sql" nome_do_banco_de_dados` : restaura o conteúdo contido no arquivo de backup em banco de dados específico (será solicitada a senha de acesso do usuário)

## Links
- More information: https://dev.mysql.com/doc/refman/en/mysqldump.html
