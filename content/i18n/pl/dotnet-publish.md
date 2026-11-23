# dotnet publish

Opublikuj aplikację .NET i jej zależności w celu wdrożenia na docelowym systemie.

## Usage
- `dotnet publish -c Release ścieżka/do/projektu_lub_solucji` : opublikuj projekt w konfiguracji wydania
- `dotnet publish -sc true -r identyfikator_runtime ścieżka/do/projektu_lub_solucji` : opublikuj projekt z dołączonym wybranym środowiskiem uruchomieniowym
- `dotnet publish -r identyfikator_runtime -p:PublishSingleFile=true ścieżka/do/projektu_lub_solucji` : zapakuj aplikację do pojedyńczego pliku uruchomieniowego dla konkretnej platformy
- `dotnet publish -sc true -r identyfikator_runtime -p:PublishTrimmed=true ścieżka/do/projektu_lub_solucji` : pomiń nieużywane biblioteki aby obniżyć rozmiar wdrażanej aplikacji
- `dotnet publish --no-restore ścieżka/do/projektu_lub_solucji` : kompiluj projekt bez przywracania zależności
- `dotnet publish -o ściezka/do/katalogu ścieżka/do/projektu_lub_solucji` : wybierz katalog docelowy

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-publish
