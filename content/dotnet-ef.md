# dotnet ef

Perform design-time development tasks for Entity Framework Core.

## Usage
- `dotnet ef database update migration` : update the database to a specified migration
- `dotnet ef database drop` : drop the database
- `dotnet ef dbcontext list` : list available `DbContext` types
- `dotnet ef dbcontext scaffold connection_string provider` : generate code for a `DbContext` and entity types for a database
- `dotnet ef migrations add name` : add a new migration
- `dotnet ef migrations remove` : remove the last migration, rolling back the code changes that were done for the latest migration
- `dotnet ef migrations list` : list available migrations
- `dotnet ef migrations script from_migration to_migration` : generate an SQL script from migrations range

## Categories
databases

## Links
- More information: https://learn.microsoft.com/ef/core/cli/dotnet
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-ef
