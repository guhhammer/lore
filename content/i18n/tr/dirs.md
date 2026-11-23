# dirs

Dizin yığını görüntüler veya üzerinde oynama yapar. Dizin yığını, `pushd` ve `popd` komutlarıyla üzerinde oynama yapılabilen, son ziyaret edilen dizinleri gösteren bir listedir.

## Usage
- `dirs` : dizin yığınını her madde arasında boşluk olacak şekilde görüntüle
- `dirs -p` : dizin yığınını her satır başı tek madde olacak şekilde görüntüle
- `dirs -v` : dizin yığınını numaralı bir liste olarak görüntüle
- `dirs -l` : dizin yığınını tilde (`~`) öneki olmadan görüntüle
- `dirs +n` : dizin yığınında 0'dan başlamak üzere yalnızca `n`. girişi göster (yalnızca Bash)
- `dirs -n` : dizin yığınında sondan itibaren 0'dan başlamak üzere yalnızca `n`. girişi göster (yalnızca Bash)
- `dirs -c` : dizin yığınını temizle

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Directory-Stack-Builtins
