# dotnet build

Buduj aplikacje .NET i ich zależności.

## Usage
- `dotnet build` : kompiluj projekt lub solucje w bieżącym katalogu
- `dotnet build ściezka/do/projektu_lub_solucji` : kompiluj w konfiguracji debugowania
- `dotnet build -c Release` : kompiluj w konfiguracji wydania
- `dotnet build --no-restore` : kompiluj bez przywracania zależności
- `dotnet build -v quiet|minimal|normal|detailed|diagnostic` : kompiluj z wybranym poziomem szczegółowości logu
- `dotnet build -r identyfikator_runtime` : kompiluj dla wybranego środowiska uruchomieniowego
- `dotnet build -o ścieżka/do/katalogu` : kompiluj do wybranego katalogu

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-build
