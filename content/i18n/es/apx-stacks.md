# apx stacks

Administra stacks en `apx`. Nota: las configuraciones de stacks creadas por el usuario se almacenan en `~/.local/share/apx/stacks`.

## Usage
- `apx stacks new` : crea de forma interactiva una nueva configuración de stack
- `apx stacks update nombre` : actualiza de forma interactiva una configuración de stack
- `apx stacks list` : muestra la lista de todas las configuraciones de stacks disponibles
- `apx stacks rm --name cadena_de_caracteres` : elimina una configuración de stack específica
- `apx stacks import --input ruta/al/stack.yml` : importa una configuración de stack
- `apx stacks export --name cadena_de_caracteres --output ruta/al/archivo_de_salida` : exporta una configuración de stack (Nota: la opción de salida es opcional, por defecto se exporta al directorio de trabajo actual)

## Links
- More information: https://docs.vanillaos.org/docs/en/apx-manpage#stacks
