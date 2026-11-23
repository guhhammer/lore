# dumpsys

Suministra información sobre los servicios del sistema Android. Este comando solo se puede usar a través de `adb shell`.

## Usage
- `dumpsys` : obtén los resultados de diagnóstico para todos los servicios del sistema
- `dumpsys service` : obtén los resultados de diagnóstico para un servicio de sistema específico
- `dumpsys -l` : enumera todos los servicios que `dumpsys` sobre los que puede proporcionar información
- `dumpsys service -h` : enumera los argumentos específicos del servicio para un servicio determinado
- `dumpsys --skip service` : excluye un servicio específico de la salida de diagnóstico
- `dumpsys -t seconds` : especifica un período de tiempo de espera en segundos (predeterminado en 10 segundos)

## Links
- More information: https://developer.android.com/tools/dumpsys
