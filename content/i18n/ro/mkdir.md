# mkdir

Creează foldere și le setează permisiunile.

## Usage
- `mkdir cale/către/director1 cale/către/director2 ...` : creează foldere specifice
- `mkdir -p cale/către/director1 cale/către/director2 ...` : creează foldere specifice și folderele părinte dacă este nevoie
- `mkdir -m u=rwx,g=rw,o=r cale/către/director1 cale/către/director2 ...` : creează foldere cu permisiuni specifice
- `mkdir -p cale/către/{a,b}/{x,y,z}/{h,i,j}` : creează mai multe foldere imbricate, recursiv
