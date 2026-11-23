# base32

Bir dosya veya standart veriyi Base32 formatında şifrele veya yalın veri çıktısı olarak deşifre et.

## Usage
- `base32 dosya/yolu` : bir dosyayı şifrele
- `base32 -w 0|76|... dosya/yolu` : kodlanmış çıktıyı belirli bir genişlikte sar (`0` sarmayı devre dışı bırakır)
- `base32 -d dosya/yolu` : bir dosyayı deşifre et
- `herhangibirkomut | base32` : `stdin`'den şifrele
- `herhangibirkomut | base32 -d` : `stdin`'den deşifre et

## Links
- More information: https://manned.org/base32
