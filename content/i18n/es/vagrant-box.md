# vagrant box

Administra cajas Vagrant (imágenes de máquinas virtuales).

## Usage
- `vagrant box list` : lista todas las cajas instaladas
- `vagrant box add hashicorp/bionic64` : añade una nueva caja
- `vagrant box add mi-caja https://example.com/mi-caja.box` : añade una caja desde una URL personalizada
- `vagrant box remove hashicorp/bionic64` : elimina una caja instalada
- `vagrant box update` : actualiza todas las cajas que están en uso en el entorno Vagrant actual
- `vagrant box update --box bento/debian-12` : actualiza una caja específica
- `vagrant box outdated` : comprueba si hay una nueva versión disponible para la caja que está usando
- `vagrant box prune` : limpia las versiones antiguas de las cajas instaladas

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/box
