# evtest

Muestra información de los controladores de dispositivos de entrada.

## Usage
- `sudo evtest` : lista todos los dispositivos de entrada detectados
- `sudo evtest /dev/input/eventnúmero` : muestra los eventos de un dispositivo de entrada específico
- `sudo evtest --grab /dev/input/eventnúmero` : agarra el dispositivo exclusivamente, evitando que otros clientes reciban eventos
- `sudo evtest --query /dev/input/eventnúmero tipo_evento código_evento` : consulta el estado de una tecla o botón específico en un dispositivo de entrada

## Links
- More information: https://manned.org/evtest
