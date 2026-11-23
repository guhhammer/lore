# base64

Bir dosya veya standart veriyi Base64 formatında şifrele veya yalın veri çıktısı olarak deşifre et.

## Usage
- `base64 dosya/yolu` : bir dosyayı şifrele
- `base64 -w 0|76|... dosya/yolu` : kodlanmış çıktıyı belirli bir genişlikte sar (`0` sarmayı devre dışı bırakır)
- `base64 -d dosya/yolu` : bir dosyayı deşifre et
- `herhangibirkomut | base64` : `stdin`'den şifrele
- `herhangibirkomut | base64 -d` : `stdin`'den deşifre et

## Links
- More information: https://manned.org/base64
