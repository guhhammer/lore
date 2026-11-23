# minikube

Запускать Kubernetes локально.

## Usage
- `minikube start` : запустить кластер
- `minikube ip` : получить IP-адрес кластера
- `minikube service my_service --url` : получить URL для доступа к сервису с именем `my_service`, доступному через порт узла
- `minikube dashboard` : открыть панель управления Kubernetes в браузере
- `minikube stop` : остановить запущенный кластер
- `minikube delete` : удалить кластер
- `minikube tunnel` : подключиться к сервисам типа LoadBalancer

## Links
- More information: https://minikube.sigs.k8s.io/docs/
