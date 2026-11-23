# chpass

Adiciona ou altera informação de usuário do banco de dados, incluindo login shell e senha.

## Usage
- `chpass` : adiciona ou altera informação de usuário do banco de dados para o usuário atual interativamente
- `chpass -s caminho/para/shell` : define uma shell de login para o usuário atual
- `chpass -s caminho/para/shell nome_do_usuário` : define uma shell de login para um usuário específico
- `su -c 'chpass -e tempo nome_do_usuário'` : altera o tempo de expiração da conta (Unix epoch)
- `su -c 'chpass -p senha_criptografada nome_do_usuário'` : altera a senha de um usuário
- `su -c 'chpass -h hostname nome_do_usuário'` : especifica hostname ou endereço de um servidor NIS para consulta
- `su -c 'chpass -d domínio nome_do_usuário'` : especifica um domínio NIS específico (nome do domínio do sistema por padrão)

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass
