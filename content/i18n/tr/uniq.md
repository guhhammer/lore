# uniq

Bir girdideki veya dosyadaki benzersiz satırları yazdır. Tekrarlanan satırları yalnızca bitişik olduklarında algıladığı için önce satırları sıralamak gerekir.

## Usage
- `sort dosya/yolu | uniq` : tüm satırları yalnızca birer kez göster
- `sort dosya/yolu | uniq -u` : yalnızca benzersiz satırları göster
- `sort dosya/yolu | uniq -d` : yalnızca yinelenen satırları göster
- `sort dosya/yolu | uniq -i` : tüm satırları büyük/küçük harfe duyarsız şekilde yalnızca birer kez göster
- `sort dosya/yolu | uniq -c` : satırları, kaç kez geçtikleriyle birlikte göster
- `sort dosya/yolu | uniq -c | sort -nr` : satırları kaç kez geçtikleriyle birlikte, en sık geçenden başlayarak sıralı göster
- `sort dosya/yolu | uniq -w 10` : benzersizlik için satırların yalnızca ilk 10 karakterini karşılaştır
- `sort dosya/yolu | uniq -s 5` : benzersizlik için satırların ilk 5 karakterinden sonraki metni karşılaştır

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html
