# npm exec

Ejecuta binarios de paquetes `npm`.

## Usage
- `npm x comando argumento1 argumento2 ...` : ejecuta el comando desde un paquete `npm` local o remoto
- `npm x --package paquete comando` : especifica el paquete explícitamente (útil si existen varios comandos con el mismo nombre)
- `npm x --no-install comando argumento1 argumento2 ...` : ejecuta un comando si existe en la ruta actual o en `node_modules/.bin`
- `npm x --quiet comando argumento1 argumento2 ...` : ejecuta un comando específico, suprimiendo cualquier salida del propio `npm`
- `npm x --help` : muestra la ayuda

## Links
- More information: https://docs.npmjs.com/cli/npm-exec/
