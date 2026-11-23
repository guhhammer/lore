# acme.sh

Script del intérprete de comandos que implementa el protocolo cliente ACME, una alternativa a `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /ruta/a/webroot` : emite un certificado utilizando el modo webroot
- `acme.sh --issue --standalone -d example.com -d www.example.com` : emite un certificado para varios dominios utilizando el modo autónomo con el puerto 80
- `acme.sh --issue --alpn -d example.com` : emite un certificado utilizando el modo TLS autónomo con el puerto 443
- `acme.sh --issue --nginx -d example.com` : emite un certificado utilizando una configuración `nginx` que está operativa
- `acme.sh --issue --apache -d example.com` : emite un certificado utilizando una configuración Apache que está operativa
- `acme.sh --issue --dns dns_cf -d *.example.com` : emite un certificado comodín (\*) utilizando un modo API DNS automático
- `acme.sh -i -d example.com --key-file /ruta/a/example.com.key --fullchain-file /ruta/a/example.com.cer --reloadcmd "systemctl force-reload nginx"` : instala archivos de certificación en las ubicaciones especificadas (útil para la renovación automática de certificados)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
