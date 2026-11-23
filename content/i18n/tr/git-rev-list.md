# git rev-list

Değişiklikleri (commit'leri) ters kronolojik sırada sırala.

## Usage
- `git rev-list HEAD` : mevcut daldaki tüm commit'leri sırala
- `git rev-list --since='2019-12-01 00:00:00' dal_ismi` : belirtilen daldaki belirtilen tarihten daha yakın olan commit'leri sırala
- `git rev-list --merges commit` : belirtilen commit'deki tüm birleştirme commit'lerini sırala
- `git rev-list tag_name..HEAD --count` : belirtilen etiketten itibaren olan commit sayılarını çıkar

## Links
- More information: https://git-scm.com/docs/git-rev-list
