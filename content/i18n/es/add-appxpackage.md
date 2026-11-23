# add-appxpackage

Una utilidad de PowerShell para agregar un paquete de aplicación firmado (`.appx`, `.msix`, `.appxbundle` y `.msixbundle`) a una cuenta de usuario.

## Usage
- `Add-AppxPackage -Path ruta\al\paquete.msix` : agrega un paquete de aplicación
- `Add-AppxPackage -Path ruta\al\paquete.msix -DependencyPath ruta\a\dependencias.msix` : agrega un paquete de aplicación con dependencias
- `Add-AppxPackage -AppInstallerFile ruta\al\app.appinstaller` : instala una aplicación utilizando el archivo del instalador de la aplicación
- `Add-AppxPackage -Path ruta\al\paquete.msix -DependencyPath ruta\a\dependencias.msix -AllowUnsigned` : agrega un paquete no firmado

## Links
- More information: https://learn.microsoft.com/powershell/module/appx/Add-AppxPackage
