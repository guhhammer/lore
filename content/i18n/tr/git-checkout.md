# git checkout

Bulunulan dalı değiştir veya çalışma ağaçlarını onar.

## Usage
- `git checkout -b dal_ismi` : yeni bir dal oluştur ve bu dala geç
- `git checkout -b dal_ismi referans` : belirtilen bir referansa (dal, uzak/dal, etiket gibi) dayanacak şekilde yeni bir dal oluştur ve bu dala geç
- `git checkout dal_ismi` : varolan yerel bir dala geç
- `git checkout -` : en son kontrol edilmiş olan dala geç
- `git checkout -t uzak_bağlantı_adresi/dal_ismi` : uzak bağlantıdaki varolan bir dala geç
- `git checkout .` : mevcut dizindeki sahnelenmemiş tüm değişiklikleri ayır (geri alma benzeri bir komut için `git reset` komutu önerilir)
- `git checkout dosya_ismi` : sahnelenmemiş değişiklikleri belirtilen dosyaya ayır
- `git checkout dal_ismi -- dosya_ismi` : mevcut dizindeki bir dosyayı, belirtilen dalda commit edilmiş sürümü ile değiştirin

## Links
- More information: https://git-scm.com/docs/git-checkout
