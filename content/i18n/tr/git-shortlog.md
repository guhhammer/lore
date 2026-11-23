# git shortlog

'git log' çıktısını özetle.

## Usage
- `git shortlog` : yapılan tüm commit'lerin yazar ismiyle alfabetik olarak guruplanmış özetini göster
- `git shortlog -n` : yapılan tüm commit'lerin en çok commit yapan yazar ismi en üstte olacak şekilde özetini göster
- `git shortlog -c` : yapılan tüm commit'lerin yazar bilgilerini (isim ve e-posta) gösterecek şekilde özetini göster
- `git shortlog HEAD~5..HEAD` : en son yapılan 5 commit'in özetini göster (sürüm aralığı belirt)
- `git shortlog -sne` : mevcut daldaki tüm kullanıcıları, e-postalarını ve yaptıkları commit sayısını göster
- `git shortlog -sne --all` : tüm dallardaki tüm kullanıcıları, e-postalarını ve yaptıkları commit sayısını göster

## Links
- More information: https://git-scm.com/docs/git-shortlog
