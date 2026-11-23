# wafw00f

Identifica y toma la huella digital de los productos de cortafuegos de aplicaciones web (WAF) que protegen un sitio web.

## Usage
- `wafw00f https://www.example.com` : comprueba si un sitio web utiliza algún WAF
- `wafw00f -a https://www.example.com` : prueba todos los WAF detectables sin detenerte en el primer resultado
- `wafw00f -p http://localhost:8080 https://www.example.com` : pasa las solicitudes a través de un proxy (como BurpSuite)
- `wafw00f -t Cloudflare|Cloudfront|Fastly|ZScaler|... https://www.example.com` : prueba un producto WAF específico (ejecutar `wafw00f --list` para obtener una lista de todos los WAF compatibles)
- `wafw00f -H ruta/a/cabeceras.txt https://www.example.com` : pasa encabezados personalizados desde un archivo
- `wafw00f -i ruta/a/urls.txt -vv` : lee las entradas de destino desde un archivo y muestra una salida detallada (varias `v` para más detalles)
- `wafw00f -l` : lista todos los WAF que se pueden detectar

## Links
- More information: https://github.com/EnableSecurity/wafw00f/wiki/Usage#arguments-list
