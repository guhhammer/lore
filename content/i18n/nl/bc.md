# bc

Een rekenmachinetaal met willekeurige precisie.

## Usage
- `bc` : start een interactieve sessie
- `bc -i -l` : start een interactieve sessie met de standaard wiskundige bibliotheek ingeschakeld
- `echo '5 / 3' | bc` : bereken een uitdrukking
- `bc pad/naar/script.bc` : voer een script uit
- `echo 'scale = 10; 5 / 3' | bc` : bereken een uitdrukking met de gespecificeerde schaal
- `echo 's|c|a|l|e(1)' | bc -l` : bereken een sinus/cosinus/arctangens/natuurlijke logaritme/exponentiële functie met behulp van `mathlib`
- `echo "define factorial(n) { if (n <= 1) return 1; return n*factorial(n-1); }; factorial(10)" | bc` : voer een inline faculteitsscript uit

## Links
- More information: https://manned.org/bc
