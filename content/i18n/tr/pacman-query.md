# pacman-query

Arch Linux paket yönetim aracı.

## Usage
- `pacman -Q` : yüklenmiş paket ve sürümleri sırala
- `pacman -Qe` : sadece özellikle indirilmiş paket ve sürümleri sırala
- `pacman -Qo dosya_ismi` : hangi paketin belirtilen dosyaya sahip olduğunu bul
- `pacman -Qi paket_ismi` : i̇ndirilmiş bir pakete dair bilgiyi görüntüle
- `pacman -Ql paket_ismi` : bir paketin içerdiği dosyaları sırala
- `pacman -Qdtq` : yetim (başka bir pakete bağlılık olarak indirilmiş ancak herhangi bir paket tarafından gerektirilmeyen) paketleri sırala
- `pacman -Qm` : mevcut depolarda bulunmayan, indirilmiş paketleri sırala
- `pacman -Qu` : miadı dolmuş paketleri sırala

## Links
- More information: https://manned.org/pacman.8
