# aws configure

Gestiona la configuración para la CLI de AWS.

## Usage
- `aws configure` : configura AWS CLI interactivamente (crea una nueva configuración o actualiza la predeterminada)
- `aws configure --profile nombre_del_perfil` : configura un perfil con nombre para la CLI de AWS de forma interactiva (crea un perfil nuevo o actualiza uno existente)
- `aws configure get nombre` : muestra el valor de una variable de configuración específica
- `aws configure get nombre --profile nombre_del_perfil` : muestra el valor de una variable de configuración en un perfil específico
- `aws configure set nombre valor` : establece el valor de una variable de configuración específica
- `aws configure set nombre valor --profile nombre_del_perfil` : establece el valor de una variable de configuración en un perfil específico
- `aws configure list` : lista las entradas de configuración
- `aws configure list --profile nombre_del_perfil` : lista las entradas de configuración para un perfil específico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/configure/
