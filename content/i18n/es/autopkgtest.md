# autopkgtest

Ejecuta pruebas sobre paquetes de Debian.

## Usage
- `autopkgtest -- null` : construye el paquete en el directorio actual y ejecuta todas las pruebas directamente en el sistema
- `autopkgtest --test-name=nombre_de_prueba -- null` : ejecuta una prueba específica para el paquete en el directorio actual
- `autopkgtest paquete -- null` : descarga y construye un paquete específico con `apt-get`, luego ejecuta todas las pruebas
- `autopkgtest -- chroot ruta/a/nuevo/directorio` : prueba el paquete en el directorio actual utilizando un nuevo directorio raíz
- `autopkgtest -B -- null` : prueba el paquete en el directorio actual sin reconstruirlo

## Links
- More information: https://manned.org/autopkgtest
