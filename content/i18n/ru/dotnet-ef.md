# dotnet ef

Выполнять задачи разработки для Entity Framework Core.

## Usage
- `dotnet ef database update миграция` : обновить базу данных до указанной миграции
- `dotnet ef database drop` : удалить базу данных
- `dotnet ef dbcontext list` : вывести список доступных типов `DbContext`
- `dotnet ef dbcontext scaffold строка_подключения провайдер` : сгенерировать код для `DbContext` и типов сущностей на основе базы данных
- `dotnet ef migrations add имя` : добавить новую миграцию
- `dotnet ef migrations remove` : удалить последнюю миграцию, откатив изменения кода, сделанные для неё
- `dotnet ef migrations list` : вывести список доступных миграций
- `dotnet ef migrations script начальная_миграция конечная_миграция` : сгенерировать SQL-скрипт из диапазона миграций

## Links
- More information: https://learn.microsoft.com/ef/core/cli/dotnet
