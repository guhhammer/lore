# docker inspect

Возвращать низкоуровневую информацию об объектах Docker.

## Usage
- `docker inspect контейнер|образ|идентификатор` : показать информацию о контейнере, образе или томе по имени или идентификатору
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' контейнер` : показать IP-адрес контейнера
- `docker inspect -f '\{\{.LogPath\}\}' контейнер` : показать путь к файлу журнала контейнера
- `docker inspect -f '\{\{.Config.Image\}\}' контейнер` : показать имя образа контейнера
- `docker inspect -f '\{\{json .Config\}\}' контейнер` : показать информацию о конфигурации в формате JSON
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' контейнер` : показать все привязки портов
- `docker inspect` : показать справку

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
