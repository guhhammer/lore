# ls

Lista innehåll i en mapp.

## Usage
- `ls -1` : lista filer radvis
- `ls -a` : lista alla filer, inklusive dolda filer
- `ls -F` : lista filer med en avslutande symbol för att indikera filtyp (mapp/, symbolisk_länk@, körbar fil*, ...)
- `ls -la` : lista alla filer i det långa formatet (behörigheter, ägare, storlek och ändringsdatum)
- `ls -lh` : lista filer i det långa formatet med storlek i läsbara enheter (KiB, MiB, GiB)
- `ls -lSR` : lista filer i det långa formatet, sorterade efter Storlek (fallande) rekursivt
- `ls -ltr` : lista filer i det långa formatet, sorterade efter tid (senast ändrad först) i omvänd ordning (äldst först)
- `ls -d */` : lista endast mappar

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
