# psexec

Ejecuta un proceso de línea de comandos en una máquina remota. Este es un comando avanzado y puede ser potencialmente peligroso.

## Usage
- `psexec \\host_remoto cmd` : ejecuta un comando usando `cmd` en un shell remoto
- `psexec \\host_remoto -u nombre_de_usuario -p contraseña` : ejecuta un comando en un host remoto (preautenticado)
- `psexec \\host_remoto cmd /c comando -an ^>ruta\al\archivo.txt` : ejecuta un comando de forma remota y envía el resultado a un archivo
- `psexec \\host_remoto -d -i nombre_del_programa` : ejecuta un programa para interactuar con los usuarios
- `psexec \\host_remoto ipconfig /all` : muestra la configuración IP del host remoto

## Links
- More information: https://learn.microsoft.com/sysinternals/downloads/psexec
