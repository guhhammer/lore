# bless

Establece la capacidad de arranque del volumen y las opciones del disco de arranque.

## Usage
- `bless --folder /Volumes/Mac OS X/System/Library/CoreServices --bootinfo --bootefi` : bendice un volumen sólo con Mac OS X o Darwin, y crea los archivos BootX y `boot.efi` según sea necesario
- `bless --mount Volumes/Mac OS --setBoot` : configura un volumen que contenga Mac OS 9 y Mac OS X para que sea el volumen activo
- `bless --netboot --server bsdp://255.255.255.255` : configura el sistema como NetBoot y localiza un servidor disponible
- `bless --info --plist` : recopila información sobre el volumen seleccionado actualmente (según lo determinado por el firmware), adecuado para la canalización a un programa capaz de analizar las listas de propiedades

## Links
- More information: https://keith.github.io/xcode-man-pages/bless.8.html
