# gh issue

GitHub sorunlarını (issue) yönet.

## Usage
- `gh issue view sorun_numarası` : belirli bir sorunu görüntüle
- `gh issue view sorun_numarası -w` : belirli bir sorunu varsayılan web tarayıcısında görüntüle
- `gh issue new -w` : varsayılan web tarayıcısında yeni bir sorun oluştur
- `gh issue ls -L 10 -l "bug"` : son 10 `bug` etiketli sorunu listele
- `gh issue ls -s closed -A kullanıcı_adı` : belirli bir kullanıcının açtığı kapalı sorunları listele
- `gh issue status -R sahip/depo` : belirli bir depoda, kullanıcıyla ilgili sorunların durumunu görüntüle
- `gh issue reopen sorun_numarası` : belirli bir sorunu yeniden aç

## Links
- More information: https://cli.github.com/manual/gh_issue
