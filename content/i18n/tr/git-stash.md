# git stash

Yerel Git düzenlemelerini geçici bir alanda sakla.

## Usage
- `git stash` : mevcut commit'lenmemiş değişiklikleri sakla
- `git stash -u` : yeni (izlenmeyen) dosyalar dahil mevcut değişiklikleri sakla
- `git stash -p` : değiştirilen dosyaların parçalarını etkileşimli şekilde seçip sakla
- `git stash list` : tüm saklananları listele
- `git stash show -p` : saklananın, oluşturulduğu andaki commit'e göre farkını yama (patch) olarak göster
- `git stash pop` : bir saklananı uygula ve uygulanması çakışmaya yol açmıyorsa onu saklananlar listesinden kaldır
- `git stash drop` : en son saklananı sil
- `git stash clear` : tüm saklananları sil

## Links
- More information: https://git-scm.com/docs/git-stash
