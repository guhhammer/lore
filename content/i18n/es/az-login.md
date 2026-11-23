# az login

Inicia sesión en Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az login` : inicia sesión de manera interactiva
- `az login --service-principal -u http://azure-cli-service-principal -p secreto -t someone.onmicrosoft.com` : inicia sesión con una entidad de servicio mediante un secreto de cliente
- `az login --service-principal -u http://azure-cli-service-principal -p ruta/al/certificado.pem -t someone.onmicrosoft.com` : inicia sesión con una entidad de servicio mediante un certificado de cliente
- `az login -i` : inicia sesión con una identidad administrada al sistema de una máquina virtual
- `az login -i -u /subscriptions/identificador_de_subscripción/resourcegroups/mi_grupo_de_recursos/providers/Microsoft.ManagedIdentity/userAssignedIdentities/mi_identificador` : inicia sesión mediante la identidad administrada al usuario de una máquina virtual

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
