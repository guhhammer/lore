# git rebase

Bir daldan başka bir dalın üstüne commit'leri tekrar temeller. Sıklıkla bir dalı commit'leriyle beraber başka bir tabana "taşımak" için kullanılır.

## Usage
- `git rebase yeni_taban_dal` : mevcut dalı belirtilen öbür dal üzerine temelle
- `git rebase -i hedef_taban_dalı_veya_commit_değeri` : commit'lerin sıralanması, çıkartılması, birleştirilmesi veya modifiye edilmesine izin vermek için tekrar temellemeyi etkileşimli olacak şekilde başlat
- `git rebase --continue` : bir birleştirme hatası tarafından durdurulan tekrar temelleme işlemini çekişen dosyaları düzenledikten sonra devam ettir
- `git rebase --skip` : birleştirme çatışmasından ötürü durdurulan tekrar temelleme işlemini çekişen commit'leri atlayarak devam ettir
- `git rebase --abort` : devam eden tekrar temelleme işlemini iptal et (örneğin birleştirmede çatışma yaşandığında)
- `git rebase --onto yeni_taban eski_taban` : mevcut dalın bir parçasını belirtilen eski tabandan yeni tabana taşı
- `git rebase -i HEAD~5` : son 3 commit'i etkileşimli olmayacak şekilde yeniden uygula
- `git rebase -X theirs dal_ismi` : herhangi bir çatışmayı çalışan dal sürümünü kurtarmak üzere otomatik olarak çöz (`theirs` argümanı burada ters anlama sahip)

## Links
- More information: https://git-scm.com/docs/git-rebase
