# apt

Debian tabanlı dağıtımlar için paket yönetim aracı. Ubuntu 16.04 ve sonraki sürümlerde interaktif kullanıldığında `apt-get` için önerilen değiştirme.

## Usage
- `sudo apt update` : kullanılabilir paket ve versiyonların listesini yenile (Bu komutu diğer `apt` komutlarından önce kullanmanız önerilir)
- `apt search paket` : belirli bir paketi arayın
- `apt show paket` : bir paketin bilgilerini gösterin
- `sudo apt install paket` : bir paket kurun veya mevcut en son sürüme güncelleyin
- `sudo apt remove paket` : bir paketi kaldırın (bunun için "purge" kullanmak, yapılandırma dosyalarını da kaldırır)
- `sudo apt upgrade` : kurulu tüm paketleri mevcut en yeni sürümlerine yükseltin
- `apt list` : tüm paketleri listeleyin
- `apt list -i` : kurulu paketleri listeleyin

## Links
- More information: https://manned.org/apt.8
