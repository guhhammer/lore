# dotnet tool

Управлять инструментами .NET и искать опубликованные инструменты в NuGet.

## Usage
- `dotnet tool install -g dotnetsay` : установить глобальный инструмент (не используйте `--global` для локальных инструментов)
- `dotnet tool restore` : установить инструменты, определённые в локальном манифесте инструментов
- `dotnet tool update -g имя_инструмента` : обновить конкретный глобальный инструмент (не используйте `--global` для локальных инструментов)
- `dotnet tool uninstall -g имя_инструмента` : удалить глобальный инструмент (не используйте `--global` для локальных инструментов)
- `dotnet tool list -g` : вывести список установленных глобальных инструментов (не используйте `--global` для локальных инструментов)
- `dotnet tool search поисковый_запрос` : найти инструменты в NuGet
- `dotnet tool -h` : показать справку

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/global-tools
