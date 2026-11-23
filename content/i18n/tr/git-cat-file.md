# git cat-file

Git depo cisimlerine dair içerik, tür ve boyut bilgisini sağla.

## Usage
- `git cat-file -s HEAD` : HEAD commit'inin byte bazında boyutunu öğren
- `git cat-file -t 8c442dc3` : belirtilen Git cisminin türünü (blob, ağaç, commit, etiket) öğren
- `git cat-file -p HEAD~2` : git objesinin içeriğini, türüne uygun olarak hoş şekilde yansıt

## Links
- More information: https://git-scm.com/docs/git-cat-file
