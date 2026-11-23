# dotnet publish

Publish a .NET application and its dependencies to a directory for deployment to a hosting system.

## Usage
- `dotnet publish -c Release path/to/project_file` : compile a .NET project in release mode
- `dotnet publish -sc true -r runtime_identifier path/to/project_file` : publish the .NET Core runtime with your application for the specified runtime
- `dotnet publish -r runtime_identifier -p:PublishSingleFile=true path/to/project_file` : package the application into a platform-specific single-file executable
- `dotnet publish -sc true -r runtime_identifier -p:PublishTrimmed=true path/to/project_file` : trim unused libraries to reduce the deployment size of an application
- `dotnet publish --no-restore path/to/project_file` : compile a .NET project without restoring dependencies
- `dotnet publish -o path/to/directory path/to/project_file` : specify the output directory

## Categories
files, packages, development

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-publish
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-publish
