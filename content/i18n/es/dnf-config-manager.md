# dnf config-manager

Gestiona las opciones de configuración y los repositorios de DNF en sistemas basados en Fedora.

## Usage
- `dnf config-manager --add-repo=url_del_repositorio` : añade (y habilita) un repositorio desde una URL
- `dnf config-manager --dump` : imprime los valores de configuración actuales
- `dnf config-manager --enable identificador_del_repositorio` : habilita un repositorio específico
- `dnf config-manager --disable identificador_del_repositorio1 identificador_del_repositorio2 ...` : deshabilita repositorios específicos
- `dnf config-manager --setopt=opción=valor` : establece una opción de configuración para un repositorio
- `dnf config-manager --help-cmd` : muestra la ayuda

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
