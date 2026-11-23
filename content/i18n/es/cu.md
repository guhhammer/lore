# cu

Llama a otro sistema y actúa como terminal de marcado/serie o realiza transferencias de archivos sin comprobación de errores.

## Usage
- `sudo cu -l /dev/ttyXYZ` : abre un puerto serie determinado
- `sudo cu -l /dev/ttyXYZ -s 115200` : abre un puerto serie determinado con una velocidad de transmisión determinada
- `sudo cu -l /dev/ttyXYZ -s 115200 -h` : abre un puerto serie determinado con una velocidad de transmisión determinada y repite los caracteres localmente (modo semidúplex)
- `sudo cu -l /dev/ttyXYZ -s 115200 --parity=even|odd|none -f --nostop` : abre un puerto serie determinado con una velocidad de transmisión determinada, paridad y sin control de flujo de hardware o software
- `<Intro><~><.>` : sale de la sesión `cu` cuando está conectado
- `cu --help` : muestra la ayuda

## Links
- More information: https://manned.org/cu
