# dotnet add package

Добавлять или обновлять ссылку на пакет .NET в файле проекта.

## Usage
- `dotnet add package пакет` : добавить пакет в проект в текущем каталоге
- `dotnet add путь/к/файлу.csproj package пакет` : добавить пакет в конкретный проект
- `dotnet add package пакет -v 1.0.0` : добавить конкретную версию пакета в проект
- `dotnet add package пакет -s https://api.nuget.org/v3/index.json` : добавить пакет, используя конкретный источник NuGet
- `dotnet add package пакет -f net7.0` : добавить пакет только для конкретной целевой платформы
- `dotnet add package пакет --package-directory путь/к/каталогу` : добавить пакет, указав каталог для восстановления пакетов (`~/.nuget/packages` по умолчанию)

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-add-package
