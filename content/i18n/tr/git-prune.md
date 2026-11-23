# git prune

Nesne veritabanından erişilemeyen tüm nesneleri budamaya yarayan git komutu. Bu komut genelde doğrudan kullanılmasa da Git gc tarafından bir iç komut olarak kullanılmaktadır.

## Usage
- `git prune -n` : git prune tarafından silinebilecek nesneleri onları silmeden raporla
- `git prune -v` : erişilemeyen nesneleri buda ve `stdout`'a budanan şeyleri görüntüle
- `git prune --progress` : erişilemeyen nesneleri budarken ilerlemeyi göster

## Links
- More information: https://git-scm.com/docs/git-prune
