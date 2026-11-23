# awk

Dosyalar üzerinde çalışmak için çok yönlü bir programlama dili.

## Usage
- `awk '{print $5}' dosya/yolu/dosya` : boşlukla ayrılmış bir dosyada beşinci sütunu (alan olarak da bilinir) yazdır
- `awk '/foo/ {print $2}' dosya/yolu/dosya` : boşlukla ayrılmış bir dosyada "foo" içeren satırların ikinci sütununu yazdır
- `awk -F ',' '{print $NF}' dosya/yolu/dosya` : alan ayırıcı olarak (boşluk yerine) virgül kullanarak dosyadaki her satırın son sütununu yazdır
- `awk '{s+=$1} END {print s}' dosya/yolu/dosya` : bir dosyanın ilk sütunundaki değerleri topla ve toplamı yazdır
- `awk 'NR%3==1' dosya/yolu/dosya` : i̇lk satırdan başlayarak her üçüncü satırı yazdır
- `awk '{if ($1 == "foo") print "Tam eşleşme foo"; else if ($1 ~ "bar") print "Kısmi eşleşme çubuğu"; else print "Baz"}' dosya/yolu/dosya` : koşullara göre farklı değerler yazdır
- `awk '($10 == value)'` : 10. sütun değerinin belirtilen değere eşit olduğu tüm satırları yazdır
- `awk '($10 >= min_value && $10 <= max_value)'` : 10. sütun değeri min ile max arasında olan tüm satırları yazdır

## Links
- More information: https://github.com/onetrueawk/awk
