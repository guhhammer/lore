# install-module

Instala módulos de PowerShell desde PowerShell Gallery, NuGet y otros repositorios.

## Usage
- `Install-Module módulo` : instala un módulo o lo actualiza a la última versión disponible
- `Install-Module módulo -RequiredVersion versión` : instala un módulo con una versión específica
- `Install-Module módulo -MinimumVersion versión` : instala un módulo no anterior a una versión específica
- `Install-Module módulo -MinimumVersion versión_mínima -MaximumVersion versión_máxima` : especifica un rango de versiones compatibles (inclusive) del módulo requerido
- `Install-Module módulo -Repository repositorio` : instala el módulo desde un repositorio específico
- `Install-Module módulo -Repository repositorio1 , repositorio2 , ...` : instala el módulo desde repositorios específicos
- `Install-Module módulo -Scope AllUsers|CurrentUser` : instala el módulo para todos/usuario actual
- `Install-Module módulo -WhatIf` : realiza una simulación para determinar qué módulos se instalarán, actualizarán o eliminarán a través de `Install-Module`

## Links
- More information: https://learn.microsoft.com/powershell/module/powershellget/install-module
