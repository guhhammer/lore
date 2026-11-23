# kubectl

Выполнять команды в кластерах Kubernetes. Некоторые подкоманды, такие как `run`, имеют собственную документацию по использованию.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : вывести подробную информацию о ресурсе
- `kubectl label pods имя unhealthy=true` : присвоить указанному поду метку `unhealthy` со значением `true`
- `kubectl get all` : вывести список всех ресурсов
- `kubectl top pods|nodes` : показать использование ресурсов (CPU/память/хранилище) узлами или подами
- `kubectl cluster-info` : показать адреса мастера и служб кластера
- `kubectl explain pods.spec.containers` : показать описание определённого поля
- `kubectl logs имя_пода` : вывести логи контейнера в поде
- `kubectl exec имя_пода -- ls /` : выполнить команду в существующем поде

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
