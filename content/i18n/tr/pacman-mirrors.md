# pacman-mirrors

Manjaro Linux için pacman aynalistesi oluşturucu. pacman-mirrors'ın çalıştırıldığı her vakit, E`sudo pacman -Syyu` komutu ile veritabanının senkronize edilmesi ve sistemin güncellenmesi gerekir.

## Usage
- `sudo pacman-mirrors --fasttrack` : varsayılan ayarlar ile bir aynalistesi oluştur
- `pacman-mirrors --status` : mevcut aynaların durumunu göster
- `pacman-mirrors --get-branch` : mevcut dalı göster
- `sudo pacman-mirrors --api --set-branch stabil|instabil|test_ediliyor` : farklı bir dala geç
- `sudo pacman-mirrors --geoip` : sadece IP adresinin bulunduğu ülkenin aynalarını kullanarak bir aynalistesi oluştur

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
