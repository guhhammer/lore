# git config

Gestiona opciones de configuración personalizadas para repositorios Git. Estas configuraciones pueden ser locales (para el repositorio actual) o globales (para el usuario actual).

## Usage
- `git config --global user.name|user.email "Tu nombre|email@example.com"` : establece globalmente tu nombre o correo electrónico (esta información es necesaria para hacer un commit en un repositorio y se incluirá en todos los commits)
- `git config -l --local|global` : lista las entradas de configuración local o global
- `git config -l --system --show-origin` : lista sólo las entradas de configuración del sistema (almacenadas en `/etc/gitconfig`), y muestra la ubicación de dicho archivo
- `git config alias.unstage` : obtén el valor de una entrada de configuración dada
- `git config --global alias.unstage "reset HEAD --"` : establece el valor global de una entrada de configuración dada
- `git config --global --unset alias.unstage` : revierte una entrada de configuración global a su valor por defecto
- `git config -e` : edita la configuración local de Git (`.git/config`) en el editor por defecto
- `git config --global -e` : edita la configuración global de Git (`~/.gitconfig` por defecto o `$XDG_CONFIG_HOME/git/config` si existe tal archivo) en el editor por defecto

## Links
- More information: https://git-scm.com/docs/git-config
