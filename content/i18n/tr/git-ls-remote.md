# git ls-remote

Çevrimiçi depolardaki isim ve URL bazlı referansları sıralamaya yarayan Git komutu. İsim veya URL girilmemişse, varsayılan dal veya çevrimiçi dalın kökeni kullanılır.

## Usage
- `git ls-remote` : varsayılan çevrimiçi depodaki tüm referansları göster
- `git ls-remote --heads` : varsayılan çevrimiçi depodaki yalnızca baş referanslarını göster
- `git ls-remote -t` : varsayılan çevrimiçi depodaki yalnızca etiket referanslarını göster
- `git ls-remote depo_adresi` : girilen isim veya URL'de bulunan çevrimiçi depodaki tüm referansları göster
- `git ls-remote depo_ismi "desen"` : bir çevrimiçi depodaki referansları belirtilen desene göre göster

## Links
- More information: https://git-scm.com/docs/git-ls-remote
