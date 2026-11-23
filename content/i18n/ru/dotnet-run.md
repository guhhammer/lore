# dotnet run

Запускать приложение .NET без явных команд компиляции или запуска.

## Usage
- `dotnet run` : запустить проект в текущем каталоге
- `dotnet run --project путь/к/файлу.csproj` : запустить конкретный проект
- `dotnet run -- аргумент1=foo аргумент2=bar ...` : запустить проект с конкретными аргументами
- `dotnet run -f net7.0` : запустить проект, используя моникер целевой платформы
- `dotnet run -a x86|x64|arm|arm64 --os win|win7|osx|linux|ios|android` : указать архитектуру и ОС, доступно начиная с .NET 6 (не используйте `--runtime` с этими параметрами)

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-run
