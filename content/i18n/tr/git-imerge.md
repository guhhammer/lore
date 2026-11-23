# git imerge

İki git dalı arasında aşamalı olarak birleştirme veya taban değiştirme işlemlerini uygula. Dallar arasındaki uyuşmazlıklar özel commitler ile bölüşülerek uyuşmazlıkları çözmek kolaylaştırılır.

## Usage
- `git imerge rebase yerine_geçilecek_dal` : imerge bazlı taban değiştirme işlemini başlat (işlemden önce tabanı değiştirilmek istenen dalı kontrol et)
- `git imerge merge birleştirilecek_dal` : imerge bazlı birleştirme işlemini başlat (işlemden önce birleştirilmek istenen dalı kontrol et)
- `git imerge diagram` : devam eden birleştirme ve taban değiştirme işlemlerinin ASCII diagramını göster
- `git imerge continue --no-edit` : uyuşmazlıkları çözdükten sonra imerge işlemine devam et (önce `git add` komutu ile uyuşmayan dosyaları ekle)
- `git imerge finish` : tüm uyuşmazlıklar çözüldükten sonra imerge işlemini sonlandır
- `git imerge remove && git checkout eski_dal` : imerge işlemini sonlandır ve belirtilen eski bir dala geri dön

## Links
- More information: https://github.com/mhagger/git-imerge
