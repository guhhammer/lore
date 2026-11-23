# docker container commit

Criar uma nova imagem a partir das alterações em um contêiner.

## Usage
- `docker commit contêiner imagem:tag` : cria uma imagem a partir de um contêiner específico
- `docker commit -c "CMD comando" contêiner imagem:tag` : aplica uma instrução `CMD` do Dockerfile à imagem criada
- `docker commit -c "ENV nome=valor" contêiner imagem:tag` : aplica uma instrução `ENV` do Dockerfile à imagem criada
- `docker commit -a "autor" contêiner imagem:tag` : cria uma imagem com um autor específico nos metadados
- `docker commit -m "comentário" contêiner imagem:tag` : cria uma imagem com um comentário específico nos metadados
- `docker commit -p false contêiner imagem:tag` : cria uma imagem sem pausar o contêiner durante o commit
- `docker commit --help` : mostra ajuda

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
