# mitmproxy

Un proxy HTTP interactivo man-in-the-middle.

## Usage
- `mitmproxy` : inicia `mitmproxy` con la configuración por defecto (escuchará en el puerto `8080`)
- `mitmproxy --listen-host dirección_ip -p puerto` : inicia `mitmproxy` con una dirección y puerto personalizados
- `mitmproxy -s ruta/a/script.py` : inicia `mitmproxy` utilizando un secuencia de comandos para procesar el tráfico
- `SSLKEYLOGFILE="ruta/al/archivo" mitmproxy` : exporta los registros con las claves maestras SSL/TLS a programas externos (wireshark, etc.)
- `mitmproxy -m regular|transparent|socks5|...` : especifica el modo de funcionamiento del servidor proxy (`regular` es el predeterminado)
- `mitmproxy --console-layout horizontal|single|vertical` : configura el diseño de la consola

## Links
- More information: https://docs.mitmproxy.org/stable/
