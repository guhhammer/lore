# bun pm pack

Создавать архив `.tgz`, содержащий файлы, которые будут опубликованы в npm (аналог `npm pack`).

## Usage
- `bun pm pack` : создать архив из текущего рабочего пространства
- `bun pm pack --dry-run` : симулировать создание архива, выполнив все этапы без записи на диск
- `bun pm pack --destination путь/к/каталогу` : сохранить архив в указанный каталог
- `bun pm pack --filename имя_файла` : указать точное имя файла архива
- `bun pm pack --ignore-scripts` : пропустить скрипты prepack, postpack и prepare
- `bun pm pack --gzip-level 5` : установить уровень сжатия `gzip` (0-9, по умолчанию: 9)
- `bun pm pack --quiet` : вывести только имя файла архива без подробных логов

## Links
- More information: https://bun.com/docs/pm/cli/pm#pack
