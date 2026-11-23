# add-appxpackage

A PowerShell utility to add a signed app package (`.appx`, `.msix`, `.appxbundle`, and `.msixbundle`) to a user account.

## Usage
- `Add-AppxPackage -Path path\to\package.msix` : add an app package
- `Add-AppxPackage -Path path\to\package.msix -DependencyPath path\to\dependencies.msix` : add an app package with dependencies
- `Add-AppxPackage -AppInstallerFile path\to\app.appinstaller` : install an app using the app installer file
- `Add-AppxPackage -Path path\to\package.msix -DependencyPath path\to\dependencies.msix -AllowUnsigned` : add an unsigned package

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, files, users, windows

## Links
- More information: https://learn.microsoft.com/powershell/module/appx/Add-AppxPackage
- tldr page: https://tldr.inbrowser.app/pages/windows/add-appxpackage
