# dotnet build

Build a .NET application and its dependencies.

## Usage
- `dotnet build` : compile the project or solution in the current directory
- `dotnet build path/to/project_or_solution` : compile a .NET project or solution in debug mode
- `dotnet build -c Release` : compile in release mode
- `dotnet build --no-restore` : compile without restoring dependencies
- `dotnet build -v quiet|minimal|normal|detailed|diagnostic` : compile with a specific verbosity level
- `dotnet build -r runtime_identifier` : compile for a specific runtime
- `dotnet build -o path/to/directory` : specify the output directory

## Categories
development, files, packages

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-build
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-build
