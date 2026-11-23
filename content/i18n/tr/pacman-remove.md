# pacman-remove

Arch Linux paket yönetim aracı.

## Usage
- `sudo pacman -Rs paket_ismi` : bir paket ve bağlılıklarını sil
- `sudo pacman -Rsn paket_ismi` : bir paketi ve onun hem bağlılıklarını, hem de konfigürasyon dosyalarını sil
- `sudo pacman -R --noconfirm paket_ismi` : bir paketi telkin olmaksızın sil
- `sudo pacman -Rsn $(pacman -Qdtq)` : yetim (başka bir pakete bağlılık olarak indirilmiş ancak herhangi bir paket tarafından gerektirilmeyen) paketleri sil
- `sudo pacman -Rc paket_ismi` : bir paketi ve ona bağlı olan tüm öbür paketleri sil
- `pacman -Rp paket_ismi` : (Bir paketin silinme durumunda) Etkilenecek paketleri (silmeden) listele
- `pacman -Rh` : bu alt komut için yardım göster

## Links
- More information: https://manned.org/pacman.8
