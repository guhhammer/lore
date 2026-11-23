# add-appxpackage

Un utilitaire PowerShell pour ajouter un paquet d'applications signé (`.appx`, `.msix`, `.appxbundle`, `.appxbundle` et `.msixbundle`) à un compte utilisateur.

## Usage
- `Add-AppxPackage -Path chemin\vers\paquet.msix` : ajoute un paquet d'application
- `Add-AppxPackage -Path chemin\vers\paquet.msix -DependencyPath chemin\vers\dependences.msix` : ajoute un paquet d'application avec ses dependences
- `Add-AppxPackage -AppInstallerFile chemin\vers\application.appinstaller` : installe une application en utilisant le fichier d'installation de l'application
- `Add-AppxPackage -Path chemin\vers\paquet.msix -DependencyPath chemin\vers\dependences.msix -AllowUnsigned` : ajoute un paquet non signé

## Links
- More information: https://learn.microsoft.com/powershell/module/appx/Add-AppxPackage
