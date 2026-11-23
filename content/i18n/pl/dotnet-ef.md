# dotnet ef

Narzędzia projektowania dla Entity Framework Core.

## Usage
- `dotnet ef database update migracja` : zaaktualizuj bazę danych do wybranej migracji
- `dotnet ef database drop` : wyczyść bazę danych
- `dotnet ef dbcontext list` : wyświetl dostępne `DbContext`
- `dotnet ef dbcontext scaffold connection_string dostawca` : wygeneruj kod dla `DbContext` oraz typów encji bazy danych
- `dotnet ef migrations add nazwa` : dodaj nową migrację
- `dotnet ef migrations remove` : usuń poprzednią migrację, cofa zmiany w kodzie stworzone dla poprzedniej migracji
- `dotnet ef migrations list` : wyświetl dostępne migracje
- `dotnet ef migrations script początkowa_migracja końcowa_migracja` : wygeneruj skrypt SQL dla zakresu migracji

## Links
- More information: https://learn.microsoft.com/ef/core/cli/dotnet
