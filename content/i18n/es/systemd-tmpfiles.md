# systemd-tmpfiles

Crea, elimina y limpia archivos y directorios volátiles y temporales. Este comando es invocado automáticamente en el arranque por los servicios de systemd, ejecutarlo manualmente tiende a ser innecesario.

## Usage
- `systemd-tmpfiles --create` : crea los archivos y directorios especificados en el archivo de configuración
- `systemd-tmpfiles --clean` : limpia archivos y directorios con los parámetros de edad configurados
- `systemd-tmpfiles --remove` : elimina archivos y directorios según lo especificado en el archivo de configuración
- `systemd-tmpfiles --create --user` : aplica operaciones en archivos de configuración específicos de usuario
- `systemd-tmpfiles --create --boot` : ejecuta las líneas marcadas para el inicio del arranque
