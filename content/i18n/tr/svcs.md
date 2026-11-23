# svcs

Çalışan servislere dair bilgileri sırala.

## Usage
- `svcs` : tüm çalışan servisleri sırala
- `svcs -vx` : çalışmayan servisleri sırala
- `svcs apache` : belirtilen servise dair bilgileri sırala
- `svcs -L apache` : servis kayıt dosyasının yerini göster
- `tail $(svcs -L apache)` : servis kayıt dosyasının sonunu görüntüle

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
