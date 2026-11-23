# dnf builddep

Instala dependencias para construir un paquete dado. No está predeterminado en `dnf` pero está soportado a través de `dnf-plugins-core`.

## Usage
- `dnf builddep ruta/a/especificacion.spec` : instala dependencias para un paquete dado
- `dnf builddep --skip-unavailable ruta/a/especificacion.spec` : instala dependencias para un paquete dado pero ignora los no disponibles
- `dnf builddep -D 'expresión'` : define la macro RPM a una expresión dada
- `dnf builddep --spec argumento` : define un argumento para una ruta de archivo `.spec`
- `dnf builddep --srpm argumento` : define un argumento para una ruta de archivo `.rpm`
- `dnf builddep --help-cmd` : muestra la ayuda

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/builddep.html
