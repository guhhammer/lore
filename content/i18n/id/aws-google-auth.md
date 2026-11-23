# aws-google-auth

Dapatkan kredensial sementara (STS) bagi AWS menggunakan Google Apps sebagai penyedia akun layanan terfederasi (Single Sign-On).

## Usage
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : masuk menggunakan akun SSO Google dengan data pengenal username, IDP, dan SP, kemudian atur durasi akses kredensial sementara selama satu jam ke depan
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : masuk dengan menanyakan peran (role) yang hendak digunakan dalam membuat kredensial (bila terdapat beberapa peran SAML yang tersedia)
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : selesaikan kumpulan alias bagi akun AWS
- `aws-google-auth -h` : tampilkan bantuan

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
