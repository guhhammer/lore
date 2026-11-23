# pacman-sync

Arch Linux paket yönetim aracı.

## Usage
- `sudo pacman --sync paket_ismi` : yeni bir paket indir
- `sudo pacman --sync --refresh --sysupgrade` : tüm paketleri senkronize et ve güncelle (bahsi geçen paketleri güncellemeden indirmek için `--downloadonly` eki gereklidir)
- `sudo pacman --sync --refresh --sysupgrade --noconfirm paket_ismi` : tüm paketleri güncelle ve telkin olmaksızın yeni bir tane indir
- `pacman --sync --search "arama_şablonu"` : paket veritabanını girilen ifade ile arat
- `pacman --sync --info paket_ismi` : bir paket hakkında bilgi görüntüle
- `sudo pacman --sync --refresh --sysupgrade --overwrite örnek_dosya` : bir paket güncellemesi sırasında çakışan dosyaların üstüne yaz
- `sudo pacman --sync --refresh --sysupgrade --ignore paket_ismi` : tüm paketleri senkronize et ve güncelle, ancak belli bir paketi yoksay
- `sudo pacman --sync --clean` : kullanılmayan paket ve kullanılmamış depoları çerezlerden sil (tüm paketlerin çerezlerini temizlemek için `--clean` eki iki kez kullanılmalıdır)

## Links
- More information: https://manned.org/pacman.8
