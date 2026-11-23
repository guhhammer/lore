# dotnet run

Run a .NET application without explicit compile or launch commands.

## Usage
- `dotnet run` : run the project in the current directory
- `dotnet run --project path/to/file.csproj` : run a specific project
- `dotnet run -- arg1=foo arg2=bar ...` : run the project with specific arguments
- `dotnet run -f net7.0` : run the project using a target framework moniker
- `dotnet run -a x86|x64|arm|arm64 --os win|win7|osx|linux|ios|android` : specify architecture and OS, available since .NET 6 (Don't use `--runtime` with these options)

## Categories
development, files

## Links
- More information: https://learn.microsoft.com/dotnet/core/tools/dotnet-run
- tldr page: https://tldr.inbrowser.app/pages/common/dotnet-run
