# msbuild

The Microsoft build tool for Visual Studio project solutions.

## Usage
- `msbuild` : build the first project file in the current directory
- `msbuild path/to/project_file` : build a specific project file
- `msbuild path/to/project_file /target:targets` : specify one or more semicolon-separated targets to build
- `msbuild path/to/project_file /property:name=value` : specify one or more semicolon-separated properties
- `msbuild path/to/project_file /toolsversion:version` : specify the build tools version to use
- `msbuild path/to/project_file /detailedsummary` : display detailed information at the end of the log about how the project was configured
- `msbuild /help` : display help

## Categories
development, files

## Links
- More information: https://learn.microsoft.com/visualstudio/msbuild
- tldr page: https://tldr.inbrowser.app/pages/common/msbuild
