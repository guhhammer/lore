# git revert

Öncekilerin etkilerini geri alan yeni bir commit oluştur.

## Usage
- `git revert HEAD` : en son commit'i geri al
- `git revert HEAD~4` : en son 5. commit'i geri al
- `git revert 0c01a9` : belirli bir commit'i geri al
- `git revert dal_ismi~5..dal_ismi~2` : birden fazla commit'i geri al
- `git revert -n 0c01a9..9a1743` : yeni commit'ler oluşturma, yalnızca çalışan ağacı değiştir
- `git revert --abort` : birleştirme çakışmasından sonra geri alma (revert) işlemini iptal et

## Links
- More information: https://git-scm.com/docs/git-revert
