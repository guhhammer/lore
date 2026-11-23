# openssl req

Управлять запросами на подпись сертификата (CSR) стандарта PKCS#10.

## Usage
- `openssl req -new -sha256 -key имя_файла.key -out имя_файла.csr` : сгенерировать запрос на подпись сертификата (CSR) для отправки в центр сертификации
- `openssl req -new -x509 -newkey rsa:4096 -keyout имя_файла.key -out имя_файла.cert -subj "/C=XX/CN=foobar" -days 365` : сгенерировать самоподписанный сертификат и соответствующую пару ключей, сохранив их в файл

## Links
- More information: https://docs.openssl.org/master/man1/openssl-req/
