# podman ps

Lista los contenedores Podman.

## Usage
- `podman ps` : lista los contenedores Podman actualmente en ejecución
- `podman ps -a` : lista todos los contenedores Podman (corriendo y detenidos)
- `podman ps -l` : muestra el último contenedor creado (incluye todos los estados)
- `podman ps -f "name=nombre"` : filtra los contenedores que contienen una subcadena en su nombre
- `podman ps -f "ancestor=imagen:etiqueta"` : filtra los contenedores que comparten una imagen dada como ancestro
- `podman ps -a -f "exited=código"` : filtra por código de estado de salida
- `podman ps -f "status=estado"` : filtra los contenedores por estado (created, running, removing, paused, exited y dead). Se usa el término en inglés
- `podman ps -f "volume=ruta/al/directorio" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : filtra contenedores que montan un volumen específico o tienen un volumen montado en una ruta específica

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-ps.1.html
