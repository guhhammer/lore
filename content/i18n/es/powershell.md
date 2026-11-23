# powershell

Shell de línea de comandos y lenguaje de scripting diseñado especialmente para la administración del sistema. Este comando se refiere a la versión 5.1 de PowerShell y anteriores (también conocida como Windows PowerShell heredada). Para usar la versión más nueva y multiplataforma de PowerShell (también conocida como PowerShell Core), usa `pwsh` en lugar de `powershell`.

## Usage
- `powershell` : inicia una sesión interactiva de shell
- `powershell -NoProfile` : inicia una sesión interactiva de shell sin cargar configuraciones de inicio
- `powershell -Command "echo 'powershell se está ejecutando'"` : ejecuta comandos específicos
- `powershell -File ruta/al/script.ps1` : ejecuta un script específico
- `powershell -Version versión` : inicia una sesión con una versión específica de PowerShell
- `powershell -NoExit` : evita que el shell se cierre después de ejecutar comandos de inicio
- `powershell -InputFormat Texto|XML` : describi el formato de los datos enviados a PowerShell
- `powershell -OutputFormat Texto|XML` : determina cómo se formatea una salida de PowerShell
