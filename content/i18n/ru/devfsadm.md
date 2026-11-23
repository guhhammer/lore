# devfsadm

Администрировать `/dev`. Поддерживать пространство имён `/dev`.

## Usage
- `devfsadm -c disk` : просканировать на наличие новых дисков
- `devfsadm -C -v` : очистить все висячие ссылки `/dev` и просканировать на наличие нового устройства
- `devfsadm -C -v -n` : выполнить пробный запуск - вывести, что будет изменено, без внесения изменений

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
