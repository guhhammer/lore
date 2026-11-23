# openssl x509

Управлять сертификатами X.509.

## Usage
- `openssl x509 -in имя_файла.crt -noout -text` : показать информацию о сертификате
- `openssl x509 -enddate -noout -in имя_файла.pem` : показать дату истечения срока действия сертификата
- `openssl x509 -inform der -outform pem -in исходный_файл_сертификата -out конвертированный_файл_сертификата` : конвертировать сертификат между двоичной кодировкой DER и текстовой кодировкой PEM
- `openssl x509 -in файл_сертификата -noout -pubkey -out выходной_файл` : сохранить публичный ключ сертификата в файл

## Links
- More information: https://docs.openssl.org/master/man1/openssl-x509/
