# ls

Dizin içeriğini listele.

## Usage
- `ls -1` : dosyaları satır başına bir tane olacak şekilde listele
- `ls -a` : gizli dosyalar dahil tüm dosyaları listele
- `ls -F` : dosyaları, türünü belirten bir son ek simgesiyle listele (dizin/, sembolik_bağlantı@, çalıştırılabilir*, ...)
- `ls -la` : tüm dosyaları uzun biçimde listele (izinler, sahiplik, boyut ve değiştirilme tarihi)
- `ls -lh` : dosyaları uzun biçimde, boyutları okunabilir birimlerle (KiB, MiB, GiB) göstererek listele
- `ls -lSR` : dosyaları uzun biçimde, boyuta göre (azalan) sıralayarak özyinelemeli olarak listele
- `ls -ltr` : dosyaları uzun biçimde, değiştirilme zamanına göre ve ters sırada (en eski önce) listele
- `ls -d */` : yalnızca dizinleri listele

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
