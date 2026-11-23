# kubectl delete

Удалять ресурсы Kubernetes.

## Usage
- `kubectl delete po имя_пода` : удалить конкретный под
- `kubectl delete deploy имя_развёртывания` : удалить конкретное развёртывание
- `kubectl delete no имя_узла` : удалить конкретный узел
- `kubectl delete po --all -n пространство_имён` : удалить все поды в указанном пространстве имён
- `kubectl delete deploy,svc --all -n пространство_имён` : удалить все развёртывания и сервисы в указанном пространстве имён
- `kubectl delete no --all` : удалить все узлы
- `kubectl delete -f путь/к/манифесту.yaml` : удалить ресурсы, определённые в YAML-манифесте

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_delete/
