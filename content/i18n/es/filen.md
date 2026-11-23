# filen

Interfaz con Filen, un servicio de almacenamiento en la nube cifrado de extremo a extremo.

## Usage
- `filen` : entra en modo interactivo
- `filen upload ruta/al/archivo_local id_carpeta_remota` : sube un archivo local a una carpeta remota específica
- `filen download id_remoto ruta/a/destino_local` : descarga un archivo o carpeta utilizando su ID remoto
- `filen ls carpeta_remota` : lista archivos y carpetas dentro de una carpeta remota
- `filen rm id_remoto` : elimina un archivo o carpeta remoto (lo mueve a la papelera)
- `filen trash restore id_remoto` : restaura un elemento de la papelera
- `filen sync ruta/a/carpeta_local:/carpeta_remota --continuous` : sincroniza una carpeta local con una carpeta remota (sincronización bidireccional)
- `filen sync ruta/a/carpeta_local:ctl:/carpeta_remota` : descarga los cambios desde la nube a una carpeta local (sincronización unidireccional)

## Links
- More information: https://github.com/FilenCloudDienste/filen-cli
