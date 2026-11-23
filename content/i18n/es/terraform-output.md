# terraform output

Exporta datos estructurados sobre tus recursos Terraform.

## Usage
- `terraform output` : sin argumentos adicionales, `output` mostrará todas las salidas del módulo raíz
- `terraform output nombre` : genera solo un valor con un nombre específico
- `terraform output -raw` : convierte el valor de salida en una cadena sin formato (útil para scripts de shell)
- `terraform output -json` : formatea las salidas como un objeto JSON, con una clave por cada salida (útil con jq)

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/output
