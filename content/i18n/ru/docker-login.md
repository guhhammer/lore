# docker login

Входить в реестр Docker.

## Usage
- `docker login` : войти в реестр в интерактивном режиме
- `docker login -u имя_пользователя` : войти в реестр с определённым именем пользователя (будет запрошен пароль)
- `docker login -u имя_пользователя -p пароль сервер` : войти в реестр с именем пользователя и паролем
- `echo "пароль" | docker login -u имя_пользователя --password-stdin` : войти в реестр с паролем из `stdin`

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
