# grep

Düzenli ifadeler (`regex`) kullanarak dosyalardaki kalıpları bul.

## Usage
- `grep "aranan_kalıp" dosya/yolu1 dosya/yolu2 ...` : dosyalar içinde kalıp ara
- `grep -F "tam_dize" dosya/yolu` : tam bir dize ara (düzenli ifadeleri devre dışı bırakır)
- `grep -rI "aranan_kalıp" dizin/yolu` : bir dizindeki tüm dosyalarda bir kalıbı tekrarlı olarak ara, binary dosyaları göz ardı et
- `grep --context|--before-context|--after-context 3 "aranan_kalıp" dosya/yolu` : her eşleşmenin etrafında, öncesinde veya sonrasında 3 satır içerik yazdır
- `grep -Hn --color=always "aranan_kalıp" dosya/yolu` : renkli çıktı ile her eşleşme için dosya adını ve satır numarasını yazdır
- `grep -o "aranan_kalıp" dosya/yolu` : yalnızca eşleşen metni yazdır
- `cat dosya/yolu | grep -v "aranan_kalıp"` : `stdin`'den veri oku ve bir kalıpla eşleşen satırları yazdırma
- `grep -Ei "aranan_kalıp" dosya/yolu` : büyük/küçük harfe duyarsız modda genişletilmiş düzenli ifadeleri (`?`, `+`, `{}`, `()`, ve `|` destekler) kullan

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
