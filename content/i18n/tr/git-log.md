# git log

Commit geçmişini göster.

## Usage
- `git log` : mevcut olandan başlayarak mevcut çalışma ortamındaki git deposunun commit silsilesini ters kronolojik düzende göster
- `git log -p dosya/veya/dizin/konumu` : belirtilen dosya veya dizinin tarihini farklılıklarla beraber göster
- `git log --stat` : her bir commit'de hangi dosya(lar)ın değiştiğinin önizlemesini göster
- `git log --oneline --graph` : mevcut daldaki commit'lerin mesajlarının ilk satırını içeren bir çizelge göster
- `git log --oneline --decorate --all --graph` : bir depodaki commit, etiket ve dalların tamamını içeren bir çizelge göster
- `git log -i --grep aranan_ifade` : mesajları yalnızca belirtilen ifadeleri içeren commit'leri göster (büyük-küçük harfe duyarsız)
- `git log -n sayı --author "yazar"` : belirtilmiş yazardan gelen, belirtilen sayıda commit göster
- `git log --before "tarih" --after "tarih"` : i̇ki tarih arasında yapılmış commit'leri göster

## Links
- More information: https://git-scm.com/docs/git-log
