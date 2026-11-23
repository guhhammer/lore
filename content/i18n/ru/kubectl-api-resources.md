# kubectl api-resources

Выводить поддерживаемые API-ресурсы на сервере.

## Usage
- `kubectl api-resources` : вывести поддерживаемые API-ресурсы
- `kubectl api-resources -o wide` : вывести поддерживаемые API-ресурсы с подробной информацией
- `kubectl api-resources --sort-by имя` : вывести поддерживаемые API-ресурсы, отсортированные по столбцу
- `kubectl api-resources --namespaced` : вывести поддерживаемые ресурсы с пространством имён
- `kubectl api-resources --namespaced=false` : вывести поддерживаемые ресурсы без пространства имён
- `kubectl api-resources --api-group api_группа` : вывести поддерживаемые API-ресурсы с определённой API-группой
