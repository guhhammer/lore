# tofu output

Exporta datos estructurados sobre tus recursos OpenTofu.

## Usage
- `tofu output` : sin argumentos adicionales, `output` mostrará todas las salidas del módulo raíz
- `tofu output nombre` : muestra solo un valor con un nombre específico
- `tofu output -raw` : convierte el valor de salida en una cadena sin procesar (útil para un conjunto de instrucciones de la consola)
- `tofu output -json` : formatea las salidas como un objeto JSON, con una clave por salida (útil con `jq`)

## Links
- More information: https://opentofu.org/docs/cli/commands/output/
