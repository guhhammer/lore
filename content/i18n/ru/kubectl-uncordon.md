# kubectl uncordon

Помечать узел как планируемый, позволяя назначать на него новые поды.

## Usage
- `kubectl uncordon имя_узла` : пометить узел как планируемый
- `kubectl uncordon имя_узла1 имя_узла2 ...` : пометить несколько узлов как планируемые
- `kubectl uncordon имя_узла --context имя_контекста` : пометить узел как планируемый в определённом контексте
- `kubectl uncordon -l ключ_метки=значение_метки` : пометить узлы, соответствующие селектору меток, как планируемые
- `kubectl uncordon имя_узла --dry-run=none|server|client` : выполнить пробный запуск для предварительного просмотра изменений без фактического включения узлов

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_uncordon/
