# pulumi refresh

Actualiza los recursos de una pila.

## Usage
- `pulumi refresh` : compara el estado de la pila actual con el estado en el proveedor de la nube y adopta cualquier cambio en la pila actual
- `pulumi refresh --diff` : actualiza los recursos en la pila actual y muestra la operación como un diff enriquecido
- `pulumi refresh --expect-no-changes` : actualiza los recursos de la pila actual y devuelve un error si se produce algún cambio durante la actualización
- `pulumi refresh --preview-only` : solo muestra una vista previa de la actualización, pero no realiza la actualización en sí
- `pulumi refresh -s nombre_pila` : el nombre de la pila sobre la que operar (por defecto es la pila actual)
- `pulumi refresh -h` : muestra la ayuda

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_refresh/
