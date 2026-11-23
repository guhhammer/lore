# git push

Commit'leri uzak depoya yolla.

## Usage
- `git push` : mevcut daldaki yerel değişiklikleri onun uzak eşine gönder
- `git push uzak_bağlantı yerel_dal` : belirtilen daldaki yerel değişiklikleri onun uzak eşine gönder
- `git push uzak_bağlantı -u uzak_dal` : mevcut dalı bir uzak dal ismi ayarlayarak uzak depoda yayınla
- `git push --all uzak_bağlantı` : yerel dallardaki tüm değişiklikleri onların belirtilen uzak depodaki uzak eşlerine gönder
- `git push uzak_bağlantı -d uzak_dal` : uzak depodaki bir dalı sil
- `git push --prune uzak_bağlantı` : yerel eşi olmayan uzak dalları sil
- `git push --tags` : daha yzak depoda olmayan etiketleri yayınla

## Links
- More information: https://git-scm.com/docs/git-push
