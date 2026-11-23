# mv

Dosyaları ve dizinleri taşı veya yeniden adlandır.

## Usage
- `mv kaynak/yolu hedef/yolu` : hedef mevcut bir dizin değilse bir dosyayı veya dizini yeniden adlandır
- `mv kaynak/yolu mevcut_dizin/yolu` : bir dosyayı veya dizini mevcut bir dizinin içine taşı
- `mv kaynak/yolu1 kaynak/yolu2 ... mevcut_dizin/yolu` : birden fazla dosyayı, dosya adlarını değiştirmeden mevcut bir dizine taşı
- `mv -f kaynak/yolu hedef/yolu` : mevcut dosyaların üzerine yazmadan önce onay isteme
- `mv -i kaynak/yolu hedef/yolu` : dosya izinlerinden bağımsız olarak, mevcut dosyaların üzerine yazmadan önce etkileşimli şekilde onay iste
- `mv -n kaynak/yolu hedef/yolu` : hedefteki mevcut dosyaların üzerine yazma
- `mv -v kaynak/yolu hedef/yolu` : dosyaları ayrıntılı modda taşı, taşınan dosyaları göster
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t hedef_dizin/yolu` : taşınacak dosyaları harici araçlarla toplayabilmek için hedef dizini belirt

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
