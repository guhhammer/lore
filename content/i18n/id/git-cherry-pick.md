# git cherry-pick

Lakukan perubahan yang tercatat pada komit-komit saat ini menuju cabang saat ini. Gunakan `git checkout` terlebih dahulu jika hendak melakukan perubahan pada cabang lainnya.

## Usage
- `git cherry-pick komit` : lakukan perubahan menurut suatu komit terhadap cabang saat ini
- `git cherry-pick komit_awal~..komit_akhir` : lakukan perubahan berdasarkan urutan komit terhadap cabang saat ini (lihat juga `git rebase --onto`)
- `git cherry-pick komit1 komit2 ...` : lakukan perubahan berdasarkan kumpulan komit (tak berurut) terhadap cabang saat ini
- `git cherry-pick -n komit` : lakukan perubahan pada direktori kerja saat ini tanpa mencatat komit baru

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
