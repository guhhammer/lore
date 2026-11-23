# git branch

Dallar ile çalışmak için kullanılan ana Git komutu.

## Usage
- `git branch -a` : tüm dalları (yerel ve uzak bağlantıda olan) göster
- `git branch --show-current` : mevcut dalın ismini göster
- `git branch dal_ismi` : mevcut commit'e dayanarak yeni bir dal oluştur
- `git branch dal_ismi commit_değeri` : belirtilen commit'e dayanarak yeni bir dal oluştur
- `git branch -m eski_dal_ismi yeni_dal_ismi` : bir dalı yeniden adlandır
- `git branch -d dal_ismi` : yerel bir dalı sil
- `git push uzak_bağlantı -d uzak_dal_ismi` : uzaktaki bir dalı sil

## Links
- More information: https://git-scm.com/docs/git-branch
