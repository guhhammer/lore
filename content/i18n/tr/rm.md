# rm

Dosyaları veya dizinleri sil.

## Usage
- `rm dosya/yolu1 dosya/yolu2 ...` : belirtilen dosyaları sil
- `rm -f dosya/yolu1 dosya/yolu2 ...` : belirtilen dosyaları, var olmayanları yok sayarak sil
- `rm -i dosya/yolu1 dosya/yolu2 ...` : belirtilen dosyaları, silmeden önce tek tek onay isteyerek etkileşimli şekilde sil
- `rm -v dosya/yolu1 dosya/yolu2 ...` : belirtilen dosyaları, silinen dosyalar hakkında bilgi yazdırarak sil
- `rm -r dosya_veya_dizin/yolu1 dosya_veya_dizin/yolu2 ...` : belirtilen dosyaları ve dizinleri özyinelemeli olarak sil
- `rm -d dizin/yolu` : boş dizinleri sil (bu, güvenli yöntem olarak kabul edilir)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/rm-invocation.html
