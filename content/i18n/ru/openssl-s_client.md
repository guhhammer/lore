# openssl s_client

Создавать клиентские TLS-соединения.

## Usage
- `openssl s_client -connect хост:порт 2>/dev/null | openssl x509 -noout -dates` : показать даты начала и окончания действия сертификата домена
- `openssl </dev/null s_client -connect хост:порт` : показать сертификат, предоставленный SSL/TLS-сервером
- `openssl s_client -connect хост:порт -servername имя_хоста` : указать индикатор имени сервера (SNI) при подключении к SSL/TLS-серверу
- `openssl </dev/null s_client -connect хост:443 -showcerts` : показать полную цепочку сертификатов HTTPS-сервера

## Links
- More information: https://docs.openssl.org/master/man1/openssl-s_client/
