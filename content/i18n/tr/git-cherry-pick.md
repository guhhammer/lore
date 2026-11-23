# git cherry-pick

Varolan commit'ler ile getirilen yenilikleri mevcut dala uygula. Değişiklikleri başka bir dala aktarmak için, önce `git checkout` komutu kullanılmalıdır.

## Usage
- `git cherry-pick commit_ismi` : mevcut dala bir commit uygula
- `git cherry-pick ilk_commit~..son_commit` : mevcut dala belirtilmiş aralıktaki kadar commit uygula (ayrıca `git rebase --onto` komutunun araştırılması önerilir)
- `git cherry-pick commit_1 commit_2 ...` : mevcut dala birçok (ardışık olmayan) commit uygula
- `git cherry-pick -n commit` : bir commit'in değişikliklerini, herhangi bir yeni commit oluşturmadan çalışan dizine ekle

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
