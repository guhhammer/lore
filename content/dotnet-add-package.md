# dotnet add package

Add or update a .NET package reference in a project file.

## Usage
- `dotnet add package package` : add a package to the project in the current directory
- `dotnet add path/to/file.csproj package package` : add a package to a specific project
- `dotnet add package package -v 1.0.0` : add a specific version of a package to the project
- `dotnet add package package -s https://api.nuget.org/v3/index.json` : add a package using a specific NuGet source
- `dotnet add package package -f net7.0` : add a package only when targeting a specific framework
- `dotnet add package package --package-directory path/to/directory` : add and specify the directory where to restore packages (`~/.nuget/packages` by default)

## Categories
packages, files, development

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-add-package
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-add-package
