# kubectl cluster-info

Отображать информацию об адресах мастера Kubernetes и служб в кластере.

## Usage
- `kubectl cluster-info` : показать базовую информацию о кластере
- `kubectl cluster-info dump` : сделать дамп текущего состояния кластера в `stdout` (для отладки)
- `kubectl cluster-info dump --output-directory путь/к/каталогу` : сделать дамп состояния кластера в каталог
- `kubectl cluster-info --context имя_контекста` : использовать определённый контекст kubeconfig

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_cluster-info/
