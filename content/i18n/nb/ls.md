# ls

List opp innholdet i en mappe.

## Usage
- `ls -1` : list filer én per linje
- `ls -a` : list alle filer, inkludert skjulte filer
- `ls -F` : list filer med et etterstilt symbol som angir filtype (mappe/, symbolsk_lenke@, kjørbar*, ...)
- `ls -la` : list alle filer i langt format (rettigheter, eierskap, størrelse og endringsdato)
- `ls -lh` : list filer i langt format med størrelse vist i lesbare enheter (KiB, MiB, GiB)
- `ls -lSR` : list filer i langt format, sortert etter Størrelse (synkende) rekursivt
- `ls -ltr` : list filer i langt format, sortert etter tidspunkt for siste endring og i omvendt rekkefølge (eldste først)
- `ls -d */` : list kun mapper

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.html
