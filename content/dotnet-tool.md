# dotnet tool

Manage .NET tools and search published tools in NuGet.

## Usage
- `dotnet tool install -g dotnetsay` : install a global tool (don't use `--global` for local tools)
- `dotnet tool restore` : install tools defined in the local tool manifest
- `dotnet tool update -g tool_name` : update a specific global tool (don't use `--global` for local tools)
- `dotnet tool uninstall -g tool_name` : uninstall a global tool (don't use `--global` for local tools)
- `dotnet tool list -g` : list installed global tools (don't use `--global` for local tools)
- `dotnet tool search search_term` : search tools in NuGet
- `dotnet tool -h` : display help

## Categories
packages

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/global-tools
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-tool
