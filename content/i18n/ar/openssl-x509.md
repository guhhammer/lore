# openssl x509

أمر OpenSSL لإدارة شهادات X.509.

## Usage
- `openssl x509 -in filename.crt -noout -text` : عرض معلومات الشهادة
- `openssl x509 -enddate -noout -in filename.pem` : عرض تاريخ انتهاء صلاحية الشهادة
- `openssl x509 -inform der -outform pem -in original_certificate_file -out converted_certificate_file` : تحويل الشهادة بين الترميز الثنائي DER والترميز النصي PEM
- `openssl x509 -in certificate_file -noout -pubkey -out output_file` : تخزين المفتاح العام للشهادة في ملف

## Links
- More information: https://docs.openssl.org/master/man1/openssl-x509/
