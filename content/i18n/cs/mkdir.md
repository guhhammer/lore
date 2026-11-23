# mkdir

Vytváří adresáře a nastavuje jejich oprávnění.

## Usage
- `mkdir cesta/k/adresari1 cesta/k/adresari2 ...` : vytvořit konkrétní adresáře
- `mkdir -p cesta/k/adresari1 cesta/k/adresari2 ...` : vytvořit konkrétní adresáře a jejich nadřazené adresáře pokud je potřeba
- `mkdir -m u=rwx,g=rw,o=r cesta/k/adresari1 cesta/k/adresari2 ...` : vytvořit adresáře s konkrétním oprávněním
- `mkdir -p cesta/k/{a,b}/{x,y,z}/{h,i,j}` : vytvořit několik vnořených adresářů rekurzivně
