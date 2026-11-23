# dotnet build

Собирает приложение .NET и все его зависимости.

## Usage
- `dotnet build` : скомпилировать проект или решение в текущем каталоге
- `dotnet build путь/к/проекту_или_решению` : скомпилировать проект или решение .NET в режиме debug
- `dotnet build -c Release` : скомпилировать в режиме release
- `dotnet build --no-restore` : скомпилировать без восстановления зависимостей
- `dotnet build -v quiet|minimal|normal|detailed|diagnostic` : скомпилировать с заданным уровнем детализации выводимой информации
- `dotnet build -r идентификатор_среды_исполнения` : скомпилировать для заданной среды исполнения
- `dotnet build -o путь/к/каталогу` : указать целевой каталог

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-build
