# diff

Dosyaları ve dizinleri karşılaştırın.

## Usage
- `diff eski_dosya yeni_dosya` : dosyaları karşılaştır (`eski_dosya`'yı `yeni_dosya`'ya dönüştürmek için yapılan değişiklikleri listeler)
- `diff -w eski_dosya yeni_dosya` : boşlukları yok sayarak dosyaları karşılaştırın
- `diff -y eski_dosya yeni_dosya` : farkları yan yana göstererek dosyaları karşılaştırın
- `diff -u eski_dosya yeni_dosya` : farkları birleştirilmiş biçimde (`git diff` tarafından kullanıldığı gibi) göstererek dosyaları karşılaştırın
- `diff -r eski_dizin yeni_dizin` : dizinleri yinelemeli olarak karşılaştırın (farklı dosya/dizin adlarını ve dosyalarda yapılan değişiklikleri gösterir)
- `diff -r -q eski_dizin yeni_dizin` : dizinleri karşılaştırın, yalnızca farklı olan dosyaların adlarını gösterin
- `diff -a -u -N eski_dosya yeni_dosya >fark.patch` : git için iki metin dosyasının farklarından, var olmayan dosyaları ise boş olarak değerlendirerek bir yama dosyası oluşturun

## Links
- More information: https://manned.org/diff
