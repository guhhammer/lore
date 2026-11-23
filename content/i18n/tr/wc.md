# wc

Satırları, kelimeleri ve baytları say.

## Usage
- `wc -l dosya/yolu` : bir dosyadaki tüm satırları say
- `wc -w dosya/yolu` : bir dosyadaki tüm kelimeleri say
- `wc -c dosya/yolu` : bir dosyadaki tüm baytları say
- `wc -m dosya/yolu` : bir dosyadaki tüm karakterleri say (çok baytlı karakterleri dikkate alarak)
- `find . | wc` : standart girdiden (`stdin`) gelen tüm satırları, kelimeleri ve baytları say
- `wc -L dosya/yolu` : en uzun satırın uzunluğunu karakter sayısı olarak say

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
