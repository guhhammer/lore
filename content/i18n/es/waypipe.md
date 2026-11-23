# waypipe

Ejecuta aplicaciones gráficas de forma remota bajo un compositor Wayland.

## Usage
- `waypipe ssh usuario@servidor programa` : ejecuta un programa gráfico de forma remota y lo muestra de forma local
- `waypipe ssh usuario@servidor` : abre un túnel SSH para ejecutar cualquier programa de forma remota y lo muestra de forma local
- `waypipe --test-skip-vulkan ssh usuario@servidor programa` : omite la prueba de compatibilidad con Vulkan
- `waypipe -h` : muestra la ayuda

## Links
- More information: https://manned.org/waypipe
