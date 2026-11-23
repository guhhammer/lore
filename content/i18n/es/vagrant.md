# vagrant

Gestiona entornos de desarrollo ligeros, reproducibles y portátiles. Algunos subcomandos como `box`, `snapshot`, `halt`, etc. tienen su propia documentación de uso.

## Usage
- `vagrant init` : crea un `Vagrantfile` en el directorio actual con la caja base de Vagrant
- `vagrant init ubuntu/focal64` : crea un `Vagrantfile` con una caja del Registro Público de Vagrant
- `vagrant up` : inicia y aprovisiona el entorno Vagrant
- `vagrant suspend` : suspende la máquina
- `vagrant halt` : detiene la máquina
- `vagrant ssh` : se conecta a la máquina a través de SSH
- `vagrant ssh-config` : muestra el archivo de configuración SSH de la máquina Vagrant en ejecución
- `vagrant box list` : muestra todas las cajas locales

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli
