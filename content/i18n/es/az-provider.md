# az provider

Administra proveedores de recursos. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az provider register -n Microsoft.PolicyInsights` : registra un proveedor
- `az provider unregister -n Microsoft.Automation` : cancela el registro de un proveedor
- `az provider list` : lista todos los proveedores de una suscripción
- `az provider show -n Microsoft.Storage` : muestra información sobre un proveedor específico
- `az provider list --query "[?namespace=='Microsoft.Network'].resourceTypes[].resourceType"` : lista todos los tipos de recursos para un proveedor específico

## Links
- More information: https://learn.microsoft.com/cli/azure/provider
