# pulumi destroy

Elimina todos los recursos existentes en una pila.

## Usage
- `pulumi destroy` : elimina todos los recursos de la pila actual
- `pulumi destroy -s stack` : destruye todos los recursos de una pila específica
- `pulumi destroy -y` : acepta y destruye automáticamente los recursos tras la vista previa
- `pulumi destroy --exclude-protected` : excluye los recursos protegidos de la destrucción
- `pulumi destroy --remove` : elimina la pila y su archivo de configuración después de que se hayan eliminado todos los recursos de la pila
- `pulumi destroy --continue-on-error` : continua con la destrucción de los recursos, incluso si se produce un error

## Links
- More information: https://www.pulumi.com/docs/iac/cli/commands/pulumi_destroy/
