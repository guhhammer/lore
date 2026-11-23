# kubectl get

Получать объекты и ресурсы Kubernetes.

## Usage
- `kubectl get ns` : получить все пространства имён в текущем кластере
- `kubectl get no -n пространство_имён` : получить узлы в указанном пространстве имён
- `kubectl get po -n пространство_имён` : получить поды в указанном пространстве имён
- `kubectl get deploy -n пространство_имён` : получить развёртывания в указанном пространстве имён
- `kubectl get svc -n пространство_имён` : получить сервисы в указанном пространстве имён
- `kubectl get persistentvolumeclaims|secret|...` : получить другие ресурсы
- `kubectl get all -A` : получить все ресурсы во всех пространствах имён
- `kubectl get -f путь/к/манифесту.yaml` : получить объекты Kubernetes, определённые в YAML-манифесте

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_get/
