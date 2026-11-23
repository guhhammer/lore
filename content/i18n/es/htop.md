# htop

Muestra información dinámica en tiempo real sobre los procesos ejecutándose. Una versión mejorada de `top`.

## Usage
- `htop` : inicia `htop`
- `htop -u usuario` : inicia `htop` mostrando solo los procesos pertenecientes a un usuario dado
- `htop -t` : muestra procesos jerárquicamente en una vista de árbol para visibilizar las relaciones entre padres e hijos
- `htop -s criterio_de_ordenamiento` : ordena procesos especificando un `criterio_de_ordenamiento` (use `htop --sort help` para ver las opciones disponibles)
- `htop -d 50` : inicia `htop` con una espera dada entre las actualizaciones, en décimas de segundo (es decir, 50 = 5 segundos)
- `htop --readonly` : desactiva todas las funciones que modifican el sistema y los procesos
- `<F1>|<?>` : muestra comandos interactivos mientras se está ejecutando `htop`
- `<Tab>` : cambia a otro panel

## Links
- More information: https://manned.org/htop
