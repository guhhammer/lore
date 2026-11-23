# dotnet tool

Zarządzaj narzędziami .NET i szukaj narzędzi opublikowanych w repozytorium NuGet.

## Usage
- `dotnet tool install -g dotnetsay` : zainstaluj narzędzie globalne (nie używaj flagi `--global`, by użyć polecenia dla narzędzi lokalnych)
- `dotnet tool restore` : zainstaluj narzędzia zdefiniowane w lokalnym manifeście narzędzi
- `dotnet tool update -g nazwa_narzędzia` : zaktualizuj wyspecyfikowane narzędzie globalne (nie używaj flagi `--global`, by użyć polecenia dla narzędzi lokalnych)
- `dotnet tool uninstall -g nazwa_narzędzia` : odinstaluj narzędzie globalne (nie używaj flagi `--global`, by użyć polecenia dla narzędzi lokalnych)
- `dotnet tool list -g` : wyświetl zainstalowane narzędzia globalne (nie używaj flagi `--global`, by użyć polecenia dla narzędzi lokalnych)
- `dotnet tool search szukana_fraza` : szukaj narzędzi w repozytorium NuGet
- `dotnet tool -h` : wyświetl pomoc

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/global-tools
