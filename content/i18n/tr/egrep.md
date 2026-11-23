# egrep

Genişletilmiş düzenli ifadeler (`regex`) kullanarak dosyalardaki kalıpları bul. Not: Bu komut `grep --extended-regexp` için bir takma addır.

## Usage
- `egrep 'a+' dosya/yolu` : bir veya daha fazla kez yinelenen bir karakteri ara
- `egrep 'a?' dosya/yolu` : bir karakterin sıfır ya da bir kez geçtiği yerleri ara (isteğe bağlı eşleşme)
- `egrep 'a{10}' dosya/yolu` : bir karakterin art arda 10 kez yinelendiği yerleri ara
- `egrep 'a{3,7}' dosya/yolu` : bir karakterin art arda 3 ile 7 kez yinelendiği yerleri ara
- `egrep 'kedi|köpek|fare' dosya/yolu` : listelenen seçeneklerden birini ara
- `egrep 'c(a|o|u)p' dosya/yolu` : daha büyük bir kalıbın içinde listelenen seçeneklerden birini ara
- `egrep '(aeiou)+' dosya/yolu` : bir veya daha fazla kez yinelenen bir karakter grubunu ara
- `egrep [[:alnum:|:alpha:|:space:|...]] dosya/yolu` : standart karakter sınıflarını kullanarak ara (daha fazla bilgi için: <https://www.regular-expressions.info/posixbrackets.html>)

## Links
- More information: https://manned.org/egrep
