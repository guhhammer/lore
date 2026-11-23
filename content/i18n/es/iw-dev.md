# iw dev

Muestra y gestiona dispositivos inalámbricos.

## Usage
- `sudo iw dev wlanX set type monitor` : configura el dispositivo en modo monitor (la interfaz debe estar inactiva primero. Vea también: `ip link`)
- `sudo iw dev wlanX set type managed` : configura el dispositivo en modo gestionado (la interfaz debe estar inactiva primero)
- `sudo iw dev wlanX set channel nombre_canal` : configura el canal Wi-Fi del dispositivo (el dispositivo debe estar primero en modo monitor con la interfaz activa)
- `sudo iw dev wlanX set freq frec_en_mhz` : configura la frecuencia Wi-Fi del dispositivo en MHz (el dispositivo debe estar primero en modo monitor con la interfaz activa)
- `iw dev wlanX station dump` : muestra toda la información de las estaciones conocidas
- `sudo iw dev wlanX interface add "nombre_vif" type monitor addr 12:34:56:aa:bb:cc` : crea una interfaz virtual en modo monitor con una dirección MAC específica
- `sudo iw dev "nombre_vif" del` : elimina la interfaz virtual
