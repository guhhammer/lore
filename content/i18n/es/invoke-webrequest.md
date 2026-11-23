# invoke-webrequest

Realiza una solicitud HTTP/HTTPS a la Web. Nota: Este comando solo se puede utilizar a través de PowerShell.

## Usage
- `Invoke-WebRequest http://example.com -OutFile ruta\al\archivo` : descarga el contenido de una URL a un archivo
- `Invoke-WebRequest -Method Post -Body @{ name='roberto' } http://example.com/form` : envía datos codificados para formularios (solicitud POST de tipo `application/x-www-form-urlencoded`)
- `Invoke-WebRequest -Headers @{ X-My-Header = '123' } -Method PUT http://example.com` : envía una solicitud con un encabezado adicional, utilizando un método HTTP personalizado
- `Invoke-WebRequest -Body '{"name":"bob"}' -ContentType 'application/json' http://example.com/users/1234` : envía datos en formato JSON, especificando el encabezado tipo de contenido (content-type) adecuado
- `Invoke-WebRequest -Headers @{ Authorization = "Basic "+ [System.Convert]::ToBase64String([System.Text.Encoding]::ASCII.GetBytes("myusername:mypassword")) } http://example.com` : pasa un nombre de usuario y contraseña para autenticación ante el servidor
