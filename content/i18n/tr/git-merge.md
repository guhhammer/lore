# git merge

Dalları birleştir.

## Usage
- `git merge dal_ismi1 dal_ismi2 ...` : mevcut dal ile belirtilen dalları birleştir
- `git merge -e dal_ismi` : birleştirme mesajını düzenle
- `git merge --no-ff dal_ismi` : bir dalı birleştir ve birleştirme commit'i oluştur
- `git merge --squash dal_ismi` : bir dalı birleştirmenin sonucunu commit oluşturmadan sahnele
- `git merge --abort` : çakışma durumunda birleştirme işlemini iptal et
- `git merge -s strateji -X strateji_seçeneği dal_ismi` : belirli bir stratejiyi kullanarak birleştir

## Links
- More information: https://git-scm.com/docs/git-merge
