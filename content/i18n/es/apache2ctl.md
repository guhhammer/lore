# apache2ctl

Administra el servidor web Apache HTTP. Este comando viene con los sistemas operativos basados en Debian; para los basados en RHEL, consulte `httpd`.

## Usage
- `sudo apache2ctl start` : inicia el programa residente (daemon) de Apache. Muestra un mensaje si ya está en ejecución
- `sudo apache2ctl stop` : detiene el programa residente (daemon) de Apache
- `sudo apache2ctl restart` : reinicia el programa residente (daemon) de Apache
- `sudo apache2ctl -t` : prueba la sintaxis del archivo de configuración
- `sudo apache2ctl -M` : lista los módulos cargados

## Links
- More information: https://manned.org/apache2ctl
