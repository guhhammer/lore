# install-module

Install PowerShell modules from PowerShell Gallery, NuGet, and other repositories.

## Usage
- `Install-Module module` : install a module, or update it to the latest available version
- `Install-Module module -RequiredVersion version` : install a module with a specific version
- `Install-Module module -MinimumVersion version` : install a module no earlier than a specific version
- `Install-Module module -MinimumVersion minimum_version -MaximumVersion maximum_version` : specify a range of supported versions (inclusive) of the required module
- `Install-Module module -Repository repository` : install module from a specific repository
- `Install-Module module -Repository repository1 , repository2 , ...` : install module from specific repositories
- `Install-Module module -Scope AllUsers|CurrentUser` : install the module for all/current user
- `Install-Module module -WhatIf` : simulate a run to determine which modules will be installed, upgraded, or removed through `Install-Module`

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, system, packages, windows

## Links
- More information: https://learn.microsoft.com/powershell/module/powershellget/install-module
- tldr page: https://tldr.inbrowser.app/pages/windows/install-module
