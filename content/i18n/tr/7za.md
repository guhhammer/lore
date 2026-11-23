# 7za

Yüksek sıkıştırma oranına sahip bir dosya arşivleyicisi. Daha az dosya türünü desteklemesi dışında `7z` ile benzerdir, ancak çapraz platformludur.

## Usage
- `7za a yol/arsiv_dosyasi.7z yol/dosya_veya_dizin` : dosya veya dizin arşivle
- `7za a yol/sifreli_arsiv.7z -pparola -mhe=on yol/arsiv_dosyasi.7z` : mevcut bir arşivi şifrele (dosya adları dahil)
- `7za x yol/arsiv_dosyasi.7z` : orijinal dizin yapısını koruyarak bir arşivi dışarı çıxart
- `7za x yol/arsiv_dosyasi.7z -oyol/cikti_dizini` : bir arşivi belirli bir dizine dışarı çıxart
- `7za x yol/arsiv_dosyasi.7z -so` : bir arşivi `stdout`'a dışarı çıxart
- `7za a -t7z|bzip2|gzip|lzip|tar|... yol/arsiv_dosyasi.7z yol/dosya_veya_dizin` : belirli bir arşiv türünü kullanarak arşivle
- `7za l yol/arsiv_dosyasi.7z` : arşiv içeriğini listele
- `7za a yol/arsiv_dosyasi.7z -mx=0|1|3|5|7|9 yol/dosya_veya_dizin` : sıkıştırma seviyesini ayarla (daha yüksek, daha fazla sıkıştırma demektir ancak daha yavaştır)

## Links
- More information: https://manned.org/7za
