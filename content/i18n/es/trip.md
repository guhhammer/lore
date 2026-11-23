# trip

Una herramienta de diagnóstico de red. Combina la funcionalidad de `traceroute` y `ping`. Diseñada para ayudar en el análisis de problemas de red.

## Usage
- `sudo trip example.com` : uso básico con parámetros por defecto
- `trip example.com --unprivileged` : rastrea sin requerir privilegios elevados (solo plataformas soportadas)
- `sudo trip example.com --ipv6` : rastrea solo con `IPv6`
- `sudo trip example.com --protocol udp` : rastrea usando el protocolo `udp`
- `sudo trip example.com --protocol tcp --target-port 443` : utiliza el puerto de destino personalizado `443` para el rastreo `tcp`
- `sudo trip example.com --protocol udp --source-port 5000` : utiliza el puerto de origen personalizado `5000` para el seguimiento `udp`

## Links
- More information: https://trippy.rs/reference/cli/
