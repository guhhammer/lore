# chage

Gerencia informações de expiração de conta e senha do usuário.

## Usage
- `chage -l nome_do_usuario` : exibe as informações referentes a senha do usuário
- `sudo chage -M 10 nome_do_usuario` : habilita a expiração da senha do usuário em 10 dias
- `sudo chage -M -1 nome_do_usuario` : desabilita a expiração da senha do usuário
- `sudo chage -E YYYY-MM-DD nome_do_usuario` : define a data de expiração da conta do usuário
- `sudo chage -d 0 nome_do_usuario` : obriga o usuário a alterar sua senha no próximo login

## Links
- More information: https://manned.org/chage
