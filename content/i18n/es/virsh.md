# virsh

Gestiona dominios invitados `virsh`. (Nota: `guest_id` puede ser el ID, nombre o UUID del invitado). Algunos subcomandos como `list` tienen su propia documentación de uso.

## Usage
- `virsh connect qemu:///system` : se conecta a una sesión de hipervisor
- `sudo virsh net-start default` : activa una red llamada `default`
- `virsh list --all` : lista todos los dominios
- `virsh create ruta/al/archivo_de_configuración.xml` : crea un invitado a partir de un archivo de configuración
- `virsh edit invitado_id` : edita el archivo de configuración de un invitado (el editor puede cambiarse con `$EDITOR`)
- `virsh comando invitado_id` : arranca/rearranca/apaga/suspende/reanuda un invitado
- `virsh save invitado_id nombre_archivo` : guarda el estado actual de un invitado en un archivo
- `virsh destroy invitado_id && virsh undefine invitado_id` : elimina un invitado activo

## Links
- More information: https://libvirt.org/manpages/virsh.html
