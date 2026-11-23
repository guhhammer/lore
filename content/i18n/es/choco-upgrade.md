# choco upgrade

Actualiza uno o más paquetes con Chocolatey.

## Usage
- `choco upgrade paquete1 paquete2 ...` : actualiza uno o más paquetes
- `choco upgrade paquete --version versión` : actualiza a una versión específica de un paquete
- `choco upgrade all` : actualiza todos los paquetes
- `choco upgrade all --except "paquete1,paquete2,..."` : actualiza todos excepto los paquetes especificados separados por comas
- `choco upgrade paquete -y` : confirma automáticamente todos los mensajes
- `choco upgrade paquete -s url_fuente|alias` : especifica una fuente personalizada para recibir paquetes
- `choco upgrade paquete -u usuario -p contraseña` : proporciona un nombre de usuario y una contraseña para la autenticación

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
