# tree

Mevcut dizinin içeriğini ağaç biçiminde göster.

## Usage
- `tree -L num` : dosya ve dizinleri `num` değeri kadar derinlikte göster (1 olması durumunda mevcut dizin gösterilir)
- `tree -d` : yalnızca dizinleri göster
- `tree -a -C` : renklendirme açık olacak şekilde gizli dosyaları dahi göster
- `tree -i -f` : ağacın satırlarını girintiler yerine tüm yolu belirterek göster
- `tree -s -h --du` : tüm dosyaların ve dizinlerin eklenerek artan boyutlarını, insanların okuyabileceği bir biçimde göster
- `tree -P '*.txt' --prune` : ağaç hiyerarşisi içindeki dosyaları bir wildcard (`glob`) kalıbı kullanarak ve aranan özellikteki dosyalara sahip olmayan dizinleri yoksayarak göster
- `tree -P dizin_ismi --matchdirs --prune` : ağaç hiyerarşisi içindeki dizinleri bir wildcard (`glob`) kalıbı kullanarak ve istenen dizine atalığı olmayan dizinleri yoksayarak göster
- `tree -I 'dizin_ismi1|dizin_ismi2'` : ağacı belirtilen dizinleri yoksayarak göster

## Links
- More information: https://manned.org/tree
