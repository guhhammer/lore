# kubectl apply

Управлять приложениями через файлы, определяющие ресурсы Kubernetes. Создавать и обновлять ресурсы в кластере.

## Usage
- `kubectl apply -f путь/к/файлу` : применить конфигурацию к ресурсу по имени файла
- `kubectl apply -k путь/к/каталогу` : применить конфигурацию к ресурсу из `kustomization.yaml` в каталоге
- `cat pod.json | kubectl apply -f -` : применить конфигурацию к ресурсу через `stdin`
- `kubectl apply edit-last-applied -f путь/к/файлу` : отредактировать последние аннотации last-applied-configuration ресурсов в редакторе по умолчанию
- `kubectl apply set-last-applied -f путь/к/файлу` : установить последние аннотации last-applied-configuration в соответствии с содержимым файла
- `kubectl apply view-last-applied -f путь/к/файлу` : просмотреть последние аннотации last-applied-configuration по типу/имени или файлу

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_apply/
