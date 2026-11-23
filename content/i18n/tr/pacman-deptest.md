# pacman-deptest

Belirtilen her bağımlılığı kontrol eder ve sistemde henüz karşılanmamış bağımlılıkların bir listesini döndürür.

## Usage
- `pacman -T package1 package2 ...` : kurulu olmayan bağımlılıkların paket adlarını yazdır
- `pacman -T "bash>=5"` : kurulu paketin verilen minimum sürümü karşılayıp karşılamadığını kontrol et
- `pacman -T "bash>5"` : bir paketin daha yeni bir sürümünün kurulu olup olmadığını kontrol et
- `pacman -Th` : yardımı görüntüle

## Links
- More information: https://manned.org/pacman.8
