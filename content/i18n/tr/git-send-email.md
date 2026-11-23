# git send-email

Bir yama koleksiyonunu e-posta olarak gönder. Yamalar dosya, dizin veya sürüm listesi olarak tanımlanabilir.

## Usage
- `git send-email -1` : mevcut dizindeki son commit'i gönder
- `git send-email -1 commit` : belirtilen commit'i gönder
- `git send-email -10` : mevcut dizindeki belirtilen sayı kadar (örneğin 10) commit'i gönder
- `git send-email -commits_sayı --compose` : gönderilecek yama serisi için bir giriş e-posta mesajı gönder
- `git send-email -commits_sayı --annotate` : gönderilecek her bir yama için e-posta mesajını görüntüle ve düzenle

## Links
- More information: https://git-scm.com/docs/git-send-email
