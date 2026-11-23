# dotnet restore

Przywracanie zależności i narzędzi dla projektu .NET.

## Usage
- `dotnet restore` : przywróć zależności dla projektu lub solucji w bieżącym katalogu
- `dotnet restore ścieżka/do/projektu_lub_solucji` : przywróć zależności dla projektu lub solucji w wybranym katalogu
- `dotnet restore --no-http-cache` : przywróć zależnośći pomijając cache zapytań HTTP
- `dotnet restore --force` : wymuś rozwiązanie wszystkich zależności nawet jeśli poprzednie przywracanie zakończyło się sukcesem
- `dotnet restore --ignore-failed-sources` : ignoruj błędy w trakcie przywracania zależności ze źródeł
- `dotnet restore -v quiet|minimal|normal|detailed|diagnostic` : przywróć zależności z wybranym poziomem szczegółowości logów

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-restore
