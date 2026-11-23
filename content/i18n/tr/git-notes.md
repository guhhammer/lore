# git notes

Nesne notları ekle veya incele.

## Usage
- `git notes list` : tüm notları ve bağlı oldukları nesneleri sırala
- `git notes list nesne` : belirtilen nesneye bağlanan tüm notları sırala (varsayılan `HEAD`'dedir)
- `git notes show nesne` : belirtilen nesneye bağlanan tüm notları göster (varsayılan `HEAD`'dedir)
- `git notes append nesne` : belirtilen nesneye bir not ekle (varsayılan metin editörü açılır)
- `git notes append --message="messaj_yazısı"` : mesajı belirterek belirtilen nesneye bir not ekle
- `git notes edit nesne` : varolan bir notu düzenle (varsayılan `HEAD`'dedir)
- `git notes copy kaynak_nesne hedef_nesne` : bir notu bir nesneden öbürüne kopyala
- `git notes remove nesne` : belirtilen nesneye eklenen tüm notları sil

## Links
- More information: https://git-scm.com/docs/git-notes
