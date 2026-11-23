# pacman-upgrade

Arch Linux paket yöneticisi aracı.

## Usage
- `sudo pacman -U yol/paket1.pkg.tar.zst yol/paket2.pkg.tar.zst ...` : dosyalardan bir veya daha fazla paket kur
- `sudo pacman -U --noconfirm yol/paket.pkg.tar.zst` : paketi sorunsuz kur (onay istemeden)
- `sudo pacman -U --overwrite yol/dosya yol/paket.pkg.tar.zst` : paket kurulumu sırasında çakışan dosyaların üzerine yaz
- `sudo pacman -Ud yol/paket.pkg.tar.zst` : paket kurulumunda bağımlılık sürüm kontrollerini atla
- `pacman -Up yol/paket.pkg.tar.zst` : yükseltme sırasında etkilenecek paketleri getir ve yazdır (paketleri kurmaz)
- `pacman -Uh` : yardımı görüntüle

## Links
- More information: https://manned.org/pacman.8
