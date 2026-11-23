# kubectl attach

Подключаться к процессу, уже работающему внутри существующего контейнера.

## Usage
- `kubectl attach имя_пода` : получить вывод из пода
- `kubectl attach имя_пода -c имя_контейнера` : получить вывод из контейнера в указанном поде
- `kubectl attach rs/имя_набора_реплик` : получить вывод из первого пода набора реплик
- `kubectl attach имя_пода -it` : создать интерактивную сессию с подом
- `kubectl attach имя_пода -c имя_контейнера -it` : создать интерактивную сессию с определённым контейнером пода

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_attach/
