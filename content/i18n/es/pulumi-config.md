# pulumi config

Administra la configuración de una pila Pulumi.

## Usage
- `pulumi config --json` : muestra la configuración actual en formato JSON
- `pulumi config get clave` : obtiene el valor de una clave de configuración
- `pulumi config rm clave` : elimina un valor de configuración
- `cat ruta/al/archivo | pulumi config set clave` : establece un valor para una clave de configuración desde un archivo
- `pulumi config set --secret clave valor_53cr3t0` : establece un valor secreto (por ejemplo, la clave API) para una clave de configuración y almacena/muestra como texto cifrado
- `pulumi config --config-file ruta/al/archivo rm-all clave1 clave2 ...` : elimina varios valores de configuración de un archivo de configuración especificado

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_config/
