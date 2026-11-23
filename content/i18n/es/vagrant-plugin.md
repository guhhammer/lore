# vagrant plugin

Gestiona los complementos de Vagrant.

## Usage
- `vagrant plugin list` : lista todos los complementos actualmente instalados
- `vagrant plugin install vagrant_vbguest` : instala un complemento desde repositorios remotos, normalmente RubyGems
- `vagrant plugin install ruta/a/mi_complemento.gem` : instala un complemento desde un archivo local
- `vagrant plugin update` : actualiza todos los complementos instalados a su última versión
- `vagrant plugin update vagrant_vbguest` : actualiza un complemento a la última versión
- `vagrant plugin uninstall vagrant_vbguest` : desinstala un complemento específico

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/plugin
