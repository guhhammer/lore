# update-alternatives

Mantiene convenientemente enlaces simbólicos para determinar los comandos predeterminados.

## Usage
- `sudo update-alternatives --install ruta/al/enlace_simbólico comando ruta/al/comando prioridad` : agrega un enlace simbólico
- `sudo update-alternatives --config java` : configura un enlace simbólico para 'java'
- `sudo update-alternatives --remove java /opt/java/jdk1.8.0_102/bin/java` : quita un enlace simbólico
- `update-alternatives --display java` : muestra información sobre un comando específico
- `update-alternatives --get-selections` : muestra todos los comandos y su selección actual

## Links
- More information: https://manned.org/update-alternatives
