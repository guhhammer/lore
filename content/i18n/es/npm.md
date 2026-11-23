# npm

Gestor de paquetes JavaScript y Node.js. Gestiona proyectos Node.js y sus dependencias de módulos.

## Usage
- `npm init -y` : crea un archivo `package.json` con los valores por defecto (omite --yes para hacerlo de forma interactiva)
- `npm i` : descarga todos los paquetes alistados como dependencias en `package.json`
- `npm i nombre_paquete@versión` : descarga una versión específica de un paquete y lo añade a la lista de dependencias en `package.json`
- `npm i nombre_paquete -D` : descarga la última versión de un paquete y lo añade a la lista de dependencias de desarrollo en `package.json`
- `npm i nombre_paquete -g` : descarga la última versión de un paquete y lo instala globalmente
- `npm r nombre_paquete` : desinstala un paquete y lo elimina de la lista de dependencias en `package.json`
- `npm ls` : lista de dependencias instaladas localmente
- `npm ls -g --depth 0` : lista los paquetes instalados globalmente

## Links
- More information: https://docs.npmjs.com/cli/npm/
