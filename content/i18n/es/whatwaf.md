# whatwaf

Detecta y elude cortafuegos de aplicaciones web y sistemas de protección.

## Usage
- `whatwaf -u https://example.com --verbose` : detecta protección en una sola uRL, opcionalmente utiliza salida verbose
- `whatwaf -t número -l ruta/al/archivo` : detecta protección en un lista de URLs en paralelo desde un archivo (una URL por línea)
- `whatwaf --proxy http://127.0.0.1:8080 --pl ruta/al/archivo -u https://example.com` : envía peticiones a través de un proxy y utiliza una lista de carga útil personalizada desde un archivo (una carga útil por línea)
- `whatwaf --tor -p 'carga1,carga2,...' -u https://example.com` : envía peticiones a través de Tor (Tor debe estar instalado) utilizando cargas personalizadas (separadas por comas)
- `whatwaf --ra --throttle segundos --timeout segundos -P --force-ssl -u http://example.com` : utiliza un agente de usuario aleatorio, establece el estrangulamiento y el tiempo de espera, envía una solicitud POST y fuerza una conexión HTTPS
- `whatwaf --wafs` : enumera todos los WAF que se pueden detectar
- `whatwaf --tampers` : lista todos los scripts de manipulación disponibles

## Links
- More information: https://github.com/Ekultek/WhatWaf#basic-help-menu
