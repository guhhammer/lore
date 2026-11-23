# bootc switch

Apunta a una nueva referencia de imagen contenedora para arrancar.

## Usage
- `sudo bootc switch imagen` : cambia el SO base a una nueva imagen de contenedor desde un registro
- `sudo bootc switch --transport containers-storage imagen` : cambia el SO base a una nueva imagen de contenedor desde el almacenamiento local de imágenes del usuario root
- `sudo bootc switch --transport oci-archive ruta/a/imagen.tar.gz` : cambia el SO base a una nueva imagen contenedor almacenada en un tarball

## Links
- More information: https://manned.org/bootc-switch
