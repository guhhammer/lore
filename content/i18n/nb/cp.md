# cp

Kopier filer og mapper.

## Usage
- `cp sti/til/kildefil sti/til/målfil` : kopier en fil til et annet sted
- `cp sti/til/kildefil sti/til/overordnet_målmappe` : kopier en fil til en annen mappe, og behold filnavnet
- `cp -r sti/til/kildemappe sti/til/målmappe` : kopier innholdet i en mappe rekursivt til et annet sted (hvis målet finnes, kopieres mappen inn i det)
- `cp -vr sti/til/kildemappe sti/til/målmappe` : kopier en mappe rekursivt, i detaljert modus (viser filer etter hvert som de kopieres)
- `cp -t sti/til/målmappe sti/til/fil1 sti/til/fil2 ...` : kopier flere filer samtidig til en mappe
- `cp -i *.ext sti/til/målmappe` : kopier alle filer med en bestemt filendelse til et annet sted, i interaktiv modus (spør brukeren før overskriving)
- `cp -L lenke sti/til/målmappe` : følg symbolske lenker før kopiering
- `cp --parents kilde/sti/til/fil sti/til/målfil` : bruk hele stien til kildefilene, og opprett eventuelle manglende mellomliggende mapper ved kopiering

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cp-invocation.html
