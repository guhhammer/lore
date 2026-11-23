# git lfs

Git depolarındaki büyük dosyalarla çalış.

## Usage
- `git lfs install` : git LFS'i başlat
- `git lfs track '*.bin'` : belirtilen topağa uygun dosyaları izle
- `git config -f .lfsconfig lfs.url lfs_uç_nokta_url'si` : git LFS uç nokta URL'sini değiştir (LFS sunucusunun Git sunucusundan ayrı olması durumunda işlevseldir)
- `git lfs track` : i̇zlenen kalıpları sırala
- `git lfs ls-files` : commit'lenmiş izlenen dosyaları sırala
- `git lfs push --all uzak_depo_adresi dal_ismi` : tğm Git LFS nesnelerini uzak sunucuya gönder (hatayla karşılaşma durumunda faydalıdır)
- `git lfs fetch` : tüm Git LFS nesnelerini çek
- `git lfs checkout` : tüm Git LFS nesnelerini kontrol et

## Links
- More information: https://github.com/git-lfs/git-lfs/tree/main/docs
