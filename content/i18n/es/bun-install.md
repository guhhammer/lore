# bun install

Instala las dependencias de JavaScript de un proyecto a partir de `package.json`.

## Usage
- `bun i` : instala todas las dependencias que figuran en `package.json`
- `bun i nombre_paquete@versión` : instala un solo paquete (esto es un alias de `bun add`)
- `bun i -g nombre_paquete` : instala un paquete globalmente
- `bun i -p` : instala solo las dependencias de producción (omite `devDependencies`)
- `bun i --frozen-lockfile` : instala las dependencias exactamente según el archivo de bloqueo `bun.lockb` (archivo de bloqueo congelado)
- `bun i -f` : fuerza la descarga de todos los paquetes desde el registro, ignorando la caché

## Links
- More information: https://bun.com/docs/pm/cli/install
