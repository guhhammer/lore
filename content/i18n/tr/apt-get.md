# apt-get

Debian ve Ubuntu paket yönetim aracı. Paket aramak için `apt-cache` komutunu kullanın.

## Usage
- `sudo apt-get update` : kullanılabilir paket ve versiyon listesini güncelleyin (diğer `apt-get` komutlarını çalıştırmadan önce kullanmanız önerilir)
- `sudo apt-get install paket` : bir paket yükleyin veya son sürüme güncelleyin
- `sudo apt-get remove paket` : bir paketi silin
- `sudo apt-get purge paket` : bir paketi ve konfigürasyon dosyalarını silin
- `sudo apt-get upgrade` : yüklü paketlerin hepsini son sürümlerine yükseltin
- `sudo apt-get autoclean` : yerel depoyu temizleyin - kullanılmayan gereksiz paket dosyalarını (`.deb`) silin
- `sudo apt-get autoremove` : artık gerekmeyen paketleri silin
- `sudo apt-get dist-upgrade` : yüklenmiş paketleri yükseltin (`upgrade` gibi), ancak gereksiz paketleri silin ve yeni bağımlılıkları memnun edecek ek paketler kurun

## Links
- More information: https://manned.org/apt-get.8
