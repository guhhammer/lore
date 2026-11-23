# git checkout-index

Dosyaları indeksten çalışma ağacına kopyala.

## Usage
- `git checkout-index -a` : son commit'den beri silinen dosyaları geri döndür
- `git checkout-index -a -f` : son commit'den beri silinen veya değiştirilen dosyaları geri döndür
- `git checkout-index -a -f -n` : son commit'den beri değiştirilen dosyaları geri döndür ancak silinenleri yoksay
- `git checkout-index -a -f --prefix dışa/aktarılacak/dizin/` : tüm ağacın bir kopyasını belirtilen dizinde dışa aktar (sondaki eğik çizgi önemli)

## Links
- More information: https://git-scm.com/docs/git-checkout-index
