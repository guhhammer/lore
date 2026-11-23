# acme.sh

Script shell script que implementa o protocolo cliente ACME, ma alternativa para o `certbot`.

## Usage
- `acme.sh --issue -d example.com -w /caminho/para/webroot` : emite um certificado usando o modo webroot
- `acme.sh --issue --standalone -d example.com -d www.example.com` : emite um certificado para múltiplos domínios usando o modo standalone na porta 80
- `acme.sh --issue --alpn -d example.com` : emite um certificado usando o modo standaline TLS na porta 443
- `acme.sh --issue --nginx -d example.com` : emite um certificado usando uma configuração válida `nginx`
- `acme.sh --issue --apache -d example.com` : emite um certificado usando uma configuração válida Apache
- `acme.sh --issue --dns dns_cf -d *.example.com` : emite um certificado wildcard (\*) usando o modo DNS_API automático
- `acme.sh -i -d example.com --key-file /caminho/para/example.com.key --fullchain-file /caminho/para/example.com.cer --reloadcmd "systemctl force-reload nginx"` : instala os arquivos dos certificaods em um local específico (útil para renovação automática do certificado)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
