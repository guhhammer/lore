# openssl dgst

Генерировать значения дайджестов и выполнять операции подписи.

## Usage
- `openssl dgst -sha256 -binary -out выходной_файл входной_файл` : вычислить дайджест SHA256 для файла, сохранив результат в указанный файл
- `openssl dgst -sign файл_приватного_ключа -sha256 -sigopt rsa_padding_mode:pss -out выходной_файл входной_файл` : подписать файл с помощью ключа RSA, сохранив результат в указанный файл
- `openssl dgst -verify файл_публичного_ключа -signature файл_подписи -sigopt rsa_padding_mode:pss файл_подписанного_сообщения` : проверить подпись RSA
- `openssl dgst -sign файл_приватного_ключа -sha256 -out выходной_файл входной_файл` : подписать файл с помощью ключа ECDSA
- `openssl dgst -verify файл_публичного_ключа -signature файл_подписи файл_подписанного_сообщения` : проверить подпись ECDSA

## Links
- More information: https://docs.openssl.org/master/man1/openssl-dgst/
