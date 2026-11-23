# git show

Çeşitli Git nesnelerini (commit'ler, etiketler vs.) görüntüle.

## Usage
- `git show` : son commit'e dair bilgi (değer, mesaj, değişimler ve öbür metaveriler) göster
- `git show commit` : belirtilen commit'e dair bilgi göster
- `git show tag` : belirtilen etiket ile özleşen commit'e dair bilgi göster
- `git show dal~3` : dalın HEAD'indeki 3. commit'e dair bilgi göster
- `git show --oneline -s commit` : commit'in mesajını diff çıktısını önleyerek tek satırda göster
- `git show --stat commit` : yalnızca değiştirilen dosyalarla ilgili istatistik (eklenen/silinen karakterler) göster
- `git show --summary commit` : yalnızca eklenen, yeniden adlandırılan veya silinen dosyaların listesini göster
- `git show sürüm:dosya/konumu` : bir dosyanın belirtilen sürümdeki (örneğin dal, etiket veya commit) içeriğini göster

## Links
- More information: https://git-scm.com/docs/git-show
