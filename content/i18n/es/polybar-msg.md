# polybar-msg

Controla `polybar` utilizando mensajería entre procesos (IPC). Nota: IPC está desactivado por defecto y se puede habilitar configurando `enable-ipc = true` en la configuación de Polybar.

## Usage
- `polybar-msg cmd quit` : cierra la barra
- `polybar-msg cmd restart` : reinicia la barra en su lugar
- `polybar-msg cmd hide` : oculta la barra (no hace nada si la barra ya está oculta)
- `polybar-msg cmd show` : muestra la barra nuevamente (no hace nada si la barra no está oculta)
- `polybar-msg cmd toggle` : alterna entre oculto/visible
- `polybar-msg action "#nombre_módulo.nombre_acción.cadena_de_datos"` : ejecuta una acción de módulo (la cadena de datos es opcional)
- `polybar-msg -p pid cmd|action carga` : envía mensajes solo a una instancia específica de Polybar (todas las instancias por defecto)

## Links
- More information: https://polybar.readthedocs.io/en/stable/user/ipc.html
