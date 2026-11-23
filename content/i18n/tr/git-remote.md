# git remote

İzlenen depolar dizisini (uzak bağlantıları) yönet.

## Usage
- `git remote -v` : varolan uzak bağlantıların isim ve URL'leriyle bir listesini göster
- `git remote show uzak_bağlantı_ismi` : uzak bağlantı ile ilgili bilgi göster
- `git remote add uzak_bağlantı_ismi uzak_bağlantı_url'si` : uzak bağlantı ekle
- `git remote set-url uzak_bağlantı_ismi yeni_url` : uzak bağlantının URL'sini değiştir (mevcut URL'yi korumak için `--add` kullan)
- `git remote set-url uzak_bağlantı_ismi yeni_url --push` : gönderme (push) için kullanılacak uzak bağlantıyı, çekme (fetch) için kullanılandan farklı olarak ayarla
- `git remote get-url uzak_bağlantı_ismi` : uzak bağlantının URL'sini göster
- `git remote remove uzak_bağlantı_ismi` : uzak bağlantıyı sil
- `git remote rename eski_isim yeni_isim` : uzak bağlantıyı yeniden adlandır

## Links
- More information: https://git-scm.com/docs/git-remote
