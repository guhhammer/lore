# elasticsearch-certutil

Генерировать SSL-сертификаты для безопасности Elasticsearch и управлять ими.

## Usage
- `elasticsearch-certutil ca` : сгенерировать новый центр сертификации (CA) с параметрами по умолчанию
- `elasticsearch-certutil cert` : сгенерировать новый сертификат с использованием встроенного CA
- `elasticsearch-certutil cert -s --pem` : сгенерировать сертификаты в неинтерактивном режиме и вывести файлы PEM
- `elasticsearch-certutil http` : сгенерировать HTTP-сертификаты с использованием встроенного CA
- `elasticsearch-certutil transport -s` : сгенерировать транспортные сертификаты в неинтерактивном режиме
- `elasticsearch-certutil csr` : сгенерировать запрос на подпись сертификата (CSR)
- `elasticsearch-certutil password` : сгенерировать зашифрованные пароли хранилища ключей
- `elasticsearch-certutil password --pass пароль` : сгенерировать пароль хранилища ключей с указанным значением
