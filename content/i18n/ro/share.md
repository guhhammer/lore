# share

Face disponibilă o resursă locală sau un sistem de fișiere pentru a fi montat de către sisteme la distanță.

## Usage
- `share` : listează toate sistemele de fișiere partajate curent
- `share -F nfs -o rw /cale/către/director` : partajează un director cu acces de citire/scriere
- `share -F nfs -o ro /cale/către/director` : partajează un director cu acces doar pentru citire
- `share -F nfs -o rw,root=nume_gazdă /cale/către/director` : partajează un director cu opțiuni specifice (ex. permite acces root de la o gazdă specifică)
- `echo "share -F nfs -o rw /cale/către/director" >>/etc/dfs/dfstab` : face partajarea persistentă prin adăugarea intrărilor în `/etc/dfs/dfstab`

## Links
- More information: https://docs.oracle.com/cd/E36784_01/html/E36825/gntjt.html
