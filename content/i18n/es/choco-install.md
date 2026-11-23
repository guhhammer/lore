# choco install

Instala uno o más paquetes con Chocolatey.

## Usage
- `choco install paquete1 paquete2 ...` : instala uno o más paquetes
- `choco install ruta\al\archivo_de_paquetes.config` : instala paquetes desde un archivo de configuración personalizado
- `choco install path\to\file` : instala un archivo `.nuspec` o `.nupkg` específico
- `choco install paquete --version versión` : instala una versión específica de un paquete
- `choco install paquete --allow-multiple` : permite instalar varias versiones de un paquete
- `choco install paquete -y` : confirma todas las solicitudes automáticamente
- `choco install paquete -s url_fuente|alias` : especifica una fuente personalizada desde la cual recibir los paquetes
- `choco install paquete -u nombre_usuario -p contraseña` : proporciona un nombre de usuario y una contraseña para la autenticación

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
