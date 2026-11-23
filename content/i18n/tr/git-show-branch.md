# git show-branch

Dalları ve içerdikleri commit'leri göster.

## Usage
- `git show-branch dal_ismi|referans|commit` : bir daldaki son commit'lerin bir özetini göster
- `git show-branch dal_ismi|referans|commit` : çeşitli commit veya daldaki commit'lerin geçmişini karşılaştır
- `git show-branch --remotes` : tüm uzak takip dallarını karşılaştır
- `git show-branch --all` : hem yerel, hem de uzak takip dallarını karşılaştır
- `git show-branch --all --list` : tüm dallardaki son commit'leri sırala
- `git show-branch --current commit|dal_ismi|referans` : belirtilen dalı mevcut dal ile karşılaştır
- `git show-branch --sha1-name --current current|dal_ismi|referans` : bağlı isim yerine commit ismini görüntüle
- `git show-branch --more 5 commit|dal_ismi|referans commit|dal_ismi|referans ...` : commit'lerin ortak atasından sonraki commit'leri belirtilen sayı kadar görüntüle

## Links
- More information: https://git-scm.com/docs/git-show-branch
