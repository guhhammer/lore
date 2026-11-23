# docker container stop

Останавливать один или несколько запущенных контейнеров.

## Usage
- `docker stop контейнер` : остановить контейнер Docker
- `docker stop -s сигнал контейнер` : остановить контейнер, отправив определённый сигнал
- `docker stop -t секунды контейнер` : остановить контейнер, ожидая указанное количество секунд перед принудительным завершением
- `docker stop контейнер1 контейнер2 ...` : остановить один или несколько контейнеров
- `docker stop --help` : показать справку

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stop/
