# git restore

Çalışan ağaç dosyalarını onar. Git sürümü 2.23+ olmalıdır.

## Usage
- `git restore dosya/konumu` : sahnelenmemiş bir dosyayı mevcut commit'in sürümüne kavuştur
- `git restore -s commit dosya/konumu` : sahnelenmemiş bir dosyayı belirtilen commit'in sürümüne kavuştur
- `git restore :/` : i̇zlenen dosyalardaki sahnelenmemiş tüm değişiklikleri iptal et
- `git restore -S dosya/konumu` : bir dosyayı sahnelenmemiş hale getir
- `git restore -S :/` : tüm dosyaları sahnelenmemiş hale getir
- `git restore -W -S :/` : dosyalara yapılan sahnelenmiş veya sahnelenmemiş tüm değişiklikleri iptal et
- `git restore -p` : onarılacak dosya parçalarını etkileşimli olarak seç

## Links
- More information: https://git-scm.com/docs/git-restore
