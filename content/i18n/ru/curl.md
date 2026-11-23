# curl

Передавать данные с сервера или на сервер. Поддерживает большинство протоколов, включая HTTP, HTTPS, FTP, SCP и другие.

## Usage
- `curl https://example.com` : выполнить HTTP GET-запрос и вывести содержимое в `stdout`
- `curl -L -D - https://example.com` : выполнить HTTP GET-запрос, следуя перенаправлениям `3xx`, и вывести заголовки ответа и содержимое в `stdout`
- `curl -O https://example.com/filename.zip` : скачать файл, сохранив его под именем, указанным в URL
- `curl -X POST -d 'name=bob' http://example.com/form` : отправить данные формы (POST-запрос типа `application/x-www-form-urlencoded`). Используйте `--data @file_name` или `--data @'-'` для чтения из `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : отправить запрос с дополнительным заголовком, используя пользовательский HTTP-метод через прокси (например, BurpSuite), игнорируя самоподписанные сертификаты
- `curl -d '{"name":"bob"}' -H 'Content-Type: application/json' http://example.com/users/1234` : отправить данные в формате JSON, указав соответствующий заголовок Content-Type
- `curl -E client.pem --key key.pem -k https://example.com` : передать клиентский сертификат и приватный ключ для запроса, пропуская проверку сертификата
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : привязать имя хоста к определённому IP-адресу с подробным выводом (аналог редактирования `/etc/hosts` для пользовательского разрешения DNS)

## Links
- More information: https://curl.se/docs/manpage.html
