# adduser

Utilitário para criação de novos usuários.

## Usage
- `adduser nome_do_usuario` : cria um novo usuário, o seu diretório na pasta home e solicita o preenchimento da sua senha
- `adduser --no-create-home nome_do_usuario` : cria um novo usuário sem o seu diretório na pasta home
- `adduser --home caminho_da_pasta_do_usuario nome_do_usuario` : cria um novo usuário especificando a localização do seu diretório
- `adduser --shell caminho_para_o_shell nome_do_usuario` : cria um novo usuário e configura o seu shell de login
- `adduser --ingroup grupo nome_do_usuario` : cria um novo usuário e atribuí-lo a um grupo

## Links
- More information: https://manned.org/adduser
