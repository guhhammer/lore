# docker login

Fazer login em um registro do Docker.

## Usage
- `docker login` : faz login interativamente em um registro
- `docker login -u nome_de_usuário` : faz login em um registro com um nome de usuário específico (será solicitada a senha)
- `docker login -u nome_de_usuário -p senha servidor` : faz login em um registro com nome de usuário e senha
- `echo "senha" | docker login -u nome_de_usuário --password-stdin` : faz login em um registro com a senha vinda do `stdin`

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
