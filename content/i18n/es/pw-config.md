# pw-config

Enumera las rutas y secciones de configuración que utilizarán el servidor y los clientes de PipeWire.

## Usage
- `pw-config` : lista todos los archivos de configuración que se utilizarán
- `pw-config -n pipewire-pulse.conf` : lista todos los archivos de configuración que utilizará el servidor PulseAudio de PipeWire
- `pw-config -n pipewire-pulse.conf list` : lista todas las secciones de configuración utilizadas por el servidor PulseAudio de PipeWire
- `pw-config -n jack.conf list context.properties` : lista los fragmentos `context.properties` utilizados por los clientes JACK
- `pw-config -n jack.conf merge context.properties` : lista las `context.properties` fusionadas utilizadas por los clientes JACK
- `pw-config -n pipewire.conf -r merge context.modules` : lista los `context.modules` fusionados utilizados por el servidor PipeWire y reformat
- `pw-config -h` : muestra la ayuda

## Links
- More information: https://docs.pipewire.org/page_man_pw-config_1.html
