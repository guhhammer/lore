# docker swarm

Инструмент оркестрации контейнеров.

## Usage
- `docker swarm init` : инициализировать кластер swarm
- `docker swarm join-token worker|manager` : показать токен для присоединения менеджера или воркера
- `docker swarm join --token токен url_узла_менеджера:2377` : присоединить новый узел к кластеру
- `docker swarm leave` : удалить воркер из swarm (выполнить внутри узла воркера)
- `docker swarm ca` : показать текущий сертификат CA в формате PEM
- `docker swarm ca --rotate` : ротировать текущий сертификат CA и показать новый сертификат
- `docker swarm update --cert-expiry часыhминутыmсекундыs` : изменить срок действия сертификатов узлов

## Links
- More information: https://docs.docker.com/engine/swarm/
