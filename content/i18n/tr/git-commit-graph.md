# git commit-graph

Git commit-graph dosyalarını yaz ve doğrula.

## Usage
- `git commit-graph write` : dizinin yerel `.git` dizinindeki paketlenmiş commit'ler için bir commit-grafik dosyası yaz
- `git show-ref -s | git commit-graph write --stdin-commits` : erişilebilen tüm commitleri içeren bir commit-grafik dosyası yaz
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : `HEAD`'den erişilebilenlerin yanında mevcut commit-grafik dosyasındaki tüm commit'leri içeren bir commit-grafik dosyası oluştur

## Links
- More information: https://git-scm.com/docs/git-commit-graph
