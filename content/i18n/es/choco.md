# choco

El gestor de paquetes Chocolatey. Algunos subcomandos, como `install`, `upgrade` y `pin`, cuentan con su propia documentación de uso.

## Usage
- `choco install nombre_del_paquete` : instala un paquete
- `choco upgrade nombre_del_paquete` : actualiza un paquete específico ya instalado
- `choco upgrade all -y` : actualiza todos los paquetes obsoletos y confirma automáticamente todas las solicitudes
- `choco uninstall nombre_del_paquete -y` : desinstala un paquete y confirma automáticamente todas las solicitudes
- `choco search consulta` : busca paquetes por nombre o palabra clave
- `choco list` : muestra todos los paquetes instalados en el equipo
- `choco outdated` : muestra los paquetes para los que hay versiones más recientes disponibles
- `choco install nombre_del_paquete -s fuente` : instala un paquete desde una fuente específica

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/
