# dotnet restore

Восстанавливает зависимости и утилиты для проекта .NET.

## Usage
- `dotnet restore` : восстановить зависимости для проекта или решения .NET в текущем каталоге
- `dotnet restore путь/к/проекту_или_решению` : восстановить зависимости для проекта или решения .NET по заданному пути
- `dotnet restore --no-http-cache` : восстановить зависимости без кеширования HTTP-запросов
- `dotnet restore --force` : принудительно разрешать все зависимости, даже если предыдущее восстановление было успешным
- `dotnet restore --ignore-failed-sources` : восстановить зависимости, рассматривая ошибки источника пакетов как предупреждения
- `dotnet restore -v quiet|minimal|normal|detailed|diagnostic` : восстановить зависимости, используя заданный уровень детализации выводимой информации

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-restore
