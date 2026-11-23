# cp

Dosyaları ve dizinleri kopyalayın.

## Usage
- `cp yol/kaynak_dosya yol/hedef_dosya` : bir dosyayı başka bir konuma kopyalayın
- `cp yol/kaynak_dosya yol/hedef_ana_dizin` : dosya adını koruyarak bir dosyayı başka bir dizine kopyalayın
- `cp -r yol/kaynak_dizin yol/hedef_dizin` : bir dizinin içeriğini yinelemeli olarak başka bir konuma kopyalayın (hedef varsa, dizin bunun içine kopyalanır)
- `cp -vra yol/kaynak_dizin yol/hedef_dizin` : bir dizini yinelemeli olarak ayrıntılı modda (dosyaları kopyalandıkça gösterir) kopyalayın ve izinleri, zaman damgalarını ve sembolik bağlantıları koruyun
- `cp -t yol/hedef_dizin yol/dosya1 yol/dosya2 ...` : birden fazla dosyayı aynı anda bir dizine kopyalayın
- `cp -i *.uzantı yol/hedef_dizin` : belirli bir uzantıya sahip tüm dosyaları etkileşimli modda başka bir konuma kopyalayın (üzerine yazmadan önce kullanıcıya sorar)
- `cp -L bağlantı yol/hedef_dizin` : kopyalamadan önce sembolik bağlantıları takip edin
- `cp --parents kaynak/yol/dosya yol/hedef_dizin` : kaynak dosyaların komut satırında verilen yolunu kullanın ve kopyalarken eksik ara dizinleri oluşturun

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
