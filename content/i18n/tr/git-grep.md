# git grep

Belirtilen söz dizisini bir deponun geçmişi dahil tüm dosyalarında ara. Sıradan `grep` komutundaki birçok ek bu komut için de aynen geçerlidir.

## Usage
- `git grep söz_dizisi` : i̇zlenen dosyalarda belirtilen söz dizisini ara
- `git grep söz_dizisi -- file_glob_pattern` : i̇zlenen dosyalarda belirtilen desene uygun, belirtilen söz dizisini ara
- `git grep --recurse-submodules söz_dizisi` : alt modüller de dahil olmak üzere izlenen dosyalarda belirtilen söz dizisini ara
- `git grep söz_dizisi HEAD~2` : belirtilen depo geçmişinde belirtilen söz dizisini ara
- `git grep söz_dizisi $(git rev-list --all)` : belirtilen söz dizisini tüm dallarda ara

## Links
- More information: https://git-scm.com/docs/git-grep
