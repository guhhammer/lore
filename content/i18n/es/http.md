# http

HTTPie: un cliente HTTP diseñado para probar, depurar e interactuar generalmente con APIs y servidores HTTP.

## Usage
- `http https://example.com` : hace una solicitud simple GET (muestra encabezados de respuesta y contenido)
- `http -p H|B|h|b|m|Hh|Hhb|... https://example.com` : imprime partes específicas del contenido (`H`: encabezados de la solicitud, `B`: cuerpo de la solicitud, `h`: encabezados de la respuesta, `b`: cuerpo de la respuesta, `m`: metadatos de respuesta)
- `http GET|POST|HEAD|PUT|PATCH|DELETE|... --proxy http|https:http://localhost:8080|socks5://localhost:9050|... https://example.com` : especifica el método HTTP al enviar una solicitud y utiliza un proxy para interceptar la solicitud
- `http -F https://example.com 'User-Agent: Mozilla/5.0' 'Accept-Encoding: gzip'` : sigue cualquier redirección `3xx` y especifica encabezados adicionales en una solicitud
- `http -a username:password|token -A basic|digest|bearer GET|POST|... https://example.com/auth` : autentica ante un servidor utilizando diferentes métodos de autenticación
- `http --offline GET|DELETE|... https://example.com` : construye una solicitud pero no la envía (similar a un simulacro (dry-run))
- `http --session nombre_de_sesión|ruta/a/sesión.json -a usuario:contraseña https://example.com/auth API-KEY:xxx` : utiliza sesiones nombradas para encabezados personalizados persistentes, credenciales de autenticación y cookies
- `http -f POST https://example.com/upload cv@ruta/al/archivo` : sube un archivo a un formulario (el ejemplo a continuación supone que el campo del formulario es `<input type="file" name="cv" />`)

## Links
- More information: https://httpie.io/docs/cli/usage
