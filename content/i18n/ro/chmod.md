# chmod

Schimbă permisiunile de acces ale unui fișier sau folder.

## Usage
- `chmod u+x cale/către/fișier` : acordă utilizatorului proprietar al fișierului dreptul de a-l executa
- `chmod u+rw cale/către/fișier_sau_folder` : acordă utilizatorului dreptul de a realiza citire și wriere asupra unui fișier/folder
- `chmod g-x cale/către/fișier` : elimină dreptul de execuție al grupului
- `chmod a+rx cale/către/fișier` : acordă tuturor utilizatorilor (all) dreptul de realizare citire și execuție
- `chmod o=g cale/către/fișier` : acordă oricui (utilizatorilor din afara grupului proprietar) aceleași drepturi ca grupul
- `chmod o= cale/către/fișier` : elimină toate drepturile pentru oricine
- `chmod -R g+w,o+w cale/către/folder` : schimbă recursiv permisiunile, acordând grupului și comun dreptul de a scrie (w)
- `chmod -R a+rX cale/către/folder` : acordă recursiv tuturor utilizatorilor (all) drept de reaplicare. Acordă, de asemenea, drepturi de eXecuție fișierelor care au cel puțin un drept de execuție și tuturor sub-folderelor
