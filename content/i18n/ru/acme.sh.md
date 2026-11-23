# acme.sh

Скрипт оболочки, реализующий протокол ACME-клиента, альтернатива `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /путь/к/webroot` : выпустить сертификат, используя режим webroot
- `acme.sh --issue --standalone -d example.com -d www.example.com` : выпустить сертификат для нескольких доменов в автономном (standalone) режиме через порт 80
- `acme.sh --issue --alpn -d example.com` : выпустить сертификат в автономном режиме TLS через порт 443
- `acme.sh --issue --nginx -d example.com` : выпустить сертификат, используя текущую конфигурацию `nginx`
- `acme.sh --issue --apache -d example.com` : выпустить сертификат, используя текущую конфигурацию Apache
- `acme.sh --issue --dns dns_cf -d *.example.com` : выпустить wildcard-сертификат (\*), используя автоматический режим DNS API
- `acme.sh -i -d example.com --key-file /путь/к/example.com.key --fullchain-file /путь/к/example.com.cer --reloadcmd "systemctl force-reload nginx"` : установить файлы сертификата в указанные места (полезно для автоматического продления сертификатов)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
