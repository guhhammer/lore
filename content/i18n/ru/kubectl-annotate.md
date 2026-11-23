# kubectl annotate

Аннотировать ресурсы Kubernetes.

## Usage
- `kubectl annotate po имя_пода ключ=значение` : добавить аннотацию к поду
- `kubectl annotate po имя_пода ключ=значение --overwrite` : обновить аннотацию пода, перезаписав существующее значение
- `kubectl annotate po ключ=значение -l ключ_метки=значение_метки` : добавить аннотацию ко всем подам в пространстве имён с определённым селектором меток
- `kubectl annotate po имя_пода --list` : вывести список всех аннотаций пода
- `kubectl annotate po имя_пода ключ-` : удалить аннотацию с пода

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_annotate/
