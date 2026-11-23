# mkdir

Maak mappen aan en stel hun permissies in.

## Usage
- `mkdir pad/naar/map1 pad/naar/map2 ...` : maak specifieke mappen aan
- `mkdir -p pad/naar/map1 pad/naar/map2 ...` : maak specifieke mappen en hun bovenliggende mappen aan indien nodig
- `mkdir -m u=rwx,g=rw,o=r pad/naar/map1 pad/naar/map2 ...` : maak mappen aan met specifieke permissies
- `mkdir -p pad/naar/{a,b}/{x,y,z}/{h,i,j}` : maak meerdere geneste mappen recursief
