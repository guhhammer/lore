# kubectl create

Создавать ресурс из файла или из `stdin`.

## Usage
- `kubectl create -f путь/к/файлу.yml` : создать ресурс, используя файл определения ресурса
- `kubectl create -f -` : создать ресурс из `stdin`
- `kubectl create deploy имя_развёртывания --image образ` : создать развёртывание
- `kubectl create deploy имя_развёртывания --image образ --replicas количество_реплик` : создать развёртывание с репликами
- `kubectl create svc тип_сервиса имя_сервиса --tcp порт:целевой_порт` : создать сервис
- `kubectl create ns имя_пространства_имён` : создать пространство имён

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_create/
