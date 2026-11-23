# pacman

Arch Linux paket yönetim aracı.

## Usage
- `sudo pacman -Syu` : tüm paketleri senkronize et ve güncelle
- `sudo pacman -S paket_ismi` : yeni bir paket indir
- `sudo pacman -Rs paket_ismi` : bir paket ve bağlılıklarını sil
- `pacman -Ss "search_pattern"` : paket veritabanında `regex` veya anahter kelime ile ara
- `pacman -F "file_name"` : belirli bir dosyayı içeren paketleri veritabanında ara
- `pacman -Qe` : sadece özellikle belirtilen paket ve sürümleri sırala
- `pacman -Qtdq` : orphan paketleri listele (bağımlılık olarak kurulmuş ama aslında hiçbir paket tarafından gerekmeyenler)
- `sudo pacman -Scc` : paket çerezlerini boş alan açmak için temizle

## Links
- More information: https://manned.org/pacman.8
