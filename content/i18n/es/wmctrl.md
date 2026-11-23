# wmctrl

CLI para X Window Manager.

## Usage
- `wmctrl -l` : lista todas las ventanas, gestionadas por el gestor de ventanas
- `wmctrl -a título_ventana` : cambia a la primera ventana cuyo título (parcial) coincida
- `wmctrl -R título_ventana` : mueve una ventana al espacio de trabajo actual, levántala y dale foco
- `wmctrl -s número_de_espacio_de_trabajo` : cambia a un espacio de trabajo
- `wmctrl -r título_ventana -b toggle,fullscreen` : selecciona una ventana y activa la pantalla completa
- `wmctrl -r título_ventana -t número_de_espacio_de_trabajo` : selecciona una ventana y muévela a un espacio de trabajo

## Links
- More information: https://manned.org/wmctrl
