# tar

Arşivleme aracı. Dosyalar genellikle gzip veya bzip2 gibi bir sıkıştırma yöntemiyle birleştirilir.

## Usage
- `tar cf hedef.tar dosya1 dosya2 ...` : bir arşiv oluştur ve dosyaya yaz
- `tar czf hedef.tar.gz dosya1 dosya2 ...` : bir gzip arşivi oluştur ve dosyaya yaz
- `tar czf hedef.tar.gz -C dizin/yolu .` : göreceli yolları kullanarak bir gzip arşivi oluştur
- `tar xvf kaynak.tar.ext` : sıkıştırılmış bir arşiv dosyasını geçerli dizine ayrıntılı şekilde çıkar
- `tar xf kaynak.tar.ext -C dizin` : sıkıştırılmış bir arşiv dosyasını hedef dizine çıkar
- `tar caf hedef.tar.xz dosya1 dosya2 ...` : sıkıştırılmış bir arşiv oluştur ve sıkıştırma yöntemini seçmek için arşiv sonekini kullan
- `tar tvf kaynak.tar` : bir tar arşivinin içeriğini ayrıntılı olarak listele
- `tar xf kaynak.tar --wildcards "*.html"` : şablonla eşleşen dosyaları arşivden çıkar

## Links
- More information: https://www.gnu.org/software/tar/manual/tar.html
