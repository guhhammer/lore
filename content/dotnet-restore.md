# dotnet restore

Restore the dependencies and tools of a .NET project.

## Usage
- `dotnet restore` : restore dependencies for a .NET project or solution in the current directory
- `dotnet restore path/to/project_or_solution` : restore dependencies for a .NET project or solution in a specific location
- `dotnet restore --no-http-cache` : restore dependencies without caching the HTTP requests
- `dotnet restore --force` : force all dependencies to be resolved even if the last restore was successful
- `dotnet restore --ignore-failed-sources` : restore dependencies using package source failures as warnings
- `dotnet restore -v quiet|minimal|normal|detailed|diagnostic` : restore dependencies with a specific verbosity level

## Categories
packages, development, network

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-restore
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-restore
