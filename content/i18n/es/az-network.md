# az network

Administra los recursos de red de Azure. Parte de `azure-cli` (también conocido como `az`).

## Usage
- `az network list-usages` : lista los recursos de red en una región que cuenta con una cuota de suscripción
- `az network vnet list` : lista todas las redes virtuales en una suscripción
- `az network vnet create --address-prefixes 10.0.0.0/16 -n red_virtual -g nombre_del_grupo --subnet-name subred --subnet-prefixes 10.0.0.0/24` : crea una red virtual
- `az network nic update --accelerated-networking true -n nic -g grupo_de_recursos` : habilita la red acelerada para una tarjeta de interfaz de red

## Links
- More information: https://learn.microsoft.com/cli/azure/network
