# elasticsearch-keystore

Управлять защищёнными настройками (например, паролями, токенами и учётными данными), используемыми в Elasticsearch.

## Usage
- `elasticsearch-keystore create` : создать новое хранилище ключей (без защиты паролем)
- `elasticsearch-keystore create -p` : создать новое хранилище ключей с защитой паролем
- `elasticsearch-keystore add имя_настройки` : добавить настройку в интерактивном режиме
- `echo "значение_настройки" | elasticsearch-keystore add --stdin имя_настройки` : добавить настройку из `stdin`
- `elasticsearch-keystore remove имя_настройки` : удалить настройку из хранилища ключей
- `elasticsearch-keystore passwd` : изменить пароль хранилища ключей
- `elasticsearch-keystore list` : вывести список всех настроек, сохранённых в хранилище ключей
- `elasticsearch-keystore upgrade` : обновить формат хранилища ключей (после обновления Elasticsearch)
