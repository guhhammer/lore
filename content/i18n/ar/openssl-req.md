# openssl req

أمر OpenSSL لإدارة طلبات توقيع الشهادات PKCS#10.

## Usage
- `openssl req -new -sha256 -key filename.key -out filename.csr` : إنشاء طلب توقيع شهادة لإرساله إلى جهة تصديق
- `openssl req -new -x509 -newkey rsa:4096 -keyout filename.key -out filename.cert -subj "/C=XX/CN=foobar" -days 365` : إنشاء شهادة موقعة ذاتيًا وزوج مفاتيح مقابلة، وحفظهما في ملف

## Links
- More information: https://docs.openssl.org/master/man1/openssl-req/
