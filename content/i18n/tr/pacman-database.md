# pacman-database

Arch Linux paket veritabanında işlem yapmak için kullanılır. Kurulu paketlerin belirli özelliklerini değiştirir.

## Usage
- `sudo pacman -D --asdeps paket` : bir paketi dolaylı olarak kurulu olarak işaretle
- `sudo pacman -D --asexplicit paket` : bir paketi açıkça kurulu olarak işaretle
- `pacman -Dk` : tüm paket bağımlılıklarının kurulu olduğunu kontrol et
- `pacman -Dkk` : senkronizasyon veritabanını kontrol et ve indirilebilir paketlerin tüm belirtilen bağımlılıklarının mevcut olduğundan emin ol
- `pacman -Dkq` : sessiz modda kontrol et ve görüntüle (sadece hata mesajları gösterilir)
- `pacman -Dh` : yardımı görüntüle

## Links
- More information: https://manned.org/pacman.8
