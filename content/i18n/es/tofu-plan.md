# tofu plan

Genera y muestra los planes de ejecución de OpenTofu.

## Usage
- `tofu plan` : genera y muestra el plan de ejecución en el directorio actual
- `tofu plan -destroy` : muestra un plan para destruir todos los objetos remotos que existen actualmente
- `tofu plan -refresh-only` : muestra un plan para actualizar el estado de Tofu y los valores de salida
- `tofu plan -var 'nombre1=valor1' -var 'nombre2=valor2'` : especifica valores para las variables de entrada
- `tofu plan -target tipo_recurso.nombre_recurso` : centra la atención de Tofu solo en un subconjunto de recursos
- `tofu plan -json` : obtiene un plan en formato JSON
- `tofu plan -no-color >ruta/al/archivo` : escribe un plan en un archivo específico

## Links
- More information: https://opentofu.org/docs/cli/commands/plan/
