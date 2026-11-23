# git commit-tree

Commit cisimleri oluşturmaya yarayan düşük seviyeli araç.

## Usage
- `git commit-tree ağaç -m "mesaj"` : belirtilen mesaj ile bir commit cismi oluştur
- `git commit-tree ağaç -F örnek/dosya` : bir dosyadan mesaj okuyan bir commit cismi oluştur (`stdin` için `-` ekini kullan)
- `git commit-tree ağaç -m "mesaj" -S` : GPG anahtarıyla imzalanmış bir commit cismi oluştur
- `git commit-tree ağaç -m "mesaj" -p ana_commit_sha` : belirtilen ana commit cismi ile bir commit cismi oluştur

## Links
- More information: https://git-scm.com/docs/git-commit-tree
