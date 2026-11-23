# wget

Web'den dosya indirir. HTTP, HTTPS ve FTP protokollerini destekler.

## Usage
- `wget https://example.com/foo` : bir URL'nin içeriğini bir dosyaya indir (bu örnekte dosya adı "foo")
- `wget -O bar https://example.com/foo` : bir URL'nin içeriğini belirtilen dosya adıyla indir (bu örnekte "bar")
- `wget -pkw 3 https://example.com/bir_sayfa.html` : tek bir web sayfasını ve tüm kaynaklarını (betikler, stil dosyaları, görseller vb.) istekler arasında 3 saniyelik aralıklarla indir
- `wget -mnp https://example.com/bir_dizin/` : bir dizindeki ve alt dizinlerindeki tüm dosyaları indir (gömülü sayfa öğelerini indirmez)
- `wget --limit-rate 300k -t 100 https://example.com/bir_dizin/` : i̇ndirme hızını ve bağlantı yeniden deneme sayısını sınırla
- `wget --user kullanici_adi --password sifre https://example.com` : HTTP sunucusundan Temel Kimlik Doğrulama ile dosya indir (FTP için de geçerlidir)
- `wget -c https://example.com` : tamamlanmamış bir indirmeye devam et
- `wget -P dizin/yolu -i url_listesi.txt` : bir metin dosyasında saklanan tüm URL'leri belirtilen dizine indir

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
