# gnmic subscribe

Suscribirse a las actualizaciones de estado de un dispositivo de red gnmic.

## Usage
- `gnmic -a ip:puerto sub --path ruta` : suscribirse a las actualizaciones del estado objetivo bajo el subárbol de una ruta específica
- `gnmic -a ip:puerto sub --path ruta --sample-interval 30s` : suscribirse a un objetivo con un intervalo de muestra de 30s (por defecto es 10s)
- `gnmic -a ip:puerto sub --path ruta --stream-mode on-change --heartbeat-interval 1m` : suscribirse a un objetivo con intervalo de muestra y actualizaciones solamente cuando hay cambios
- `gnmic -a ip:puerto sub --path ruta --mode once` : suscribirse a un objetivo para una sola actualización
- `gnmic -a ip:puerto sub --path ruta -e json_ietf` : suscribirse a un objetivo y especificar la codificación de la respuesta (json_ietf)

## Links
- More information: https://gnmic.openconfig.net/cmd/subscribe/
