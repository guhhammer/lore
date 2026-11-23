# echo

Verilen argümanları yazdır.

## Usage
- `echo "Merhaba Dünya"` : bir metin mesajı yazdır. Not: Tırnak işaretleri isteğe bağlı
- `echo "Path değişkenim $PATH"` : ortam değişkenlerini kullanarak mesaj yazdır
- `echo -n "Merhaba Dünya"` : satır sonu karakteri olmadan mesaj yazdır
- `echo "Merhaba Dünya" >>dosya.txt` : dosyaya mesaj ekle
- `echo -e "Sütun 1\tSütun 2"` : ters eğik çizgi kaçış karakterlerinin (özel karakterlerin) yorumlanmasını etkinleştir
- `echo $?` : en son çalıştırılan komutun çıkış durumunu yazdır (Not: Windows Komut İstemi ve PowerShell’de eşdeğer komutlar sırasıyla `echo %errorlevel%` ve `$lastexitcode`'dur)
- `echo "Merhaba Dünya" | program` : metni `stdin` aracılığıyla başka bir programa aktar

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
