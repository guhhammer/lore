# deno

Un entorno de ejecución seguro para JavaScript, TypeScript y WebAssembly. Incluye gestión de dependencias usando `npm` o `jsr` y herramientas como bench, bundle, doc y coverage.

## Usage
- `deno` : inicia un REPL (shell interactivo, también conocido como Read-Eval-Print Loop)
- `deno init sample && cd sample && deno test` : inicia un nuevo proyecto llamado sample y lo prueba
- `deno run ruta/al/archivo.ts` : ejecuta un archivo de forma segura. Preguntará (si es necesario) para permitir net, read, etc
- `deno run --allow-env|--allow-net|--allow-write|--allow-all jsr:@deno/deployctl` : ejecuta un archivo con permisos explícitos o permite todo (solo si confías en la fuente)
- `deno task` : lista y ejecuta tareas desde `deno.json` o scripts desde `package.json`
- `deno install` : instala dependencias listadas en `deno.json` o `package.json` (también bloquea archivos)
- `deno check && deno fmt && deno lint --fix` : comprueba tipos, formato y lint (corrige si es posible)
- `deno compile ruta/al/archivo.ts` : compila el script, las dependencias importadas y el entorno de ejecución en un ejecutable independiente

## Links
- More information: https://docs.deno.com/runtime/reference/cli/
