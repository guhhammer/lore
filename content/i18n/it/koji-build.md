# koji build

Compila un pacchetto RPM.

## Usage
- `koji build target percorso/al/src.rpm` : compila un pacchetto da `src.rpm`
- `koji build target git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf` : compila un pacchetto da un URL SCM (Source Code Management)
- `koji build target percorso/al/src.rpm --scratch` : esegue una build di prova
- `koji build target percorso/al/src.rpm --wait` : attende la build, anche se è in esecuzione in background
- `koji build target percorso/al/src.rpm --nowait` : non attende la build
- `koji build -h` : mostra l'aiuto

## Links
- More information: https://docs.pagure.org/koji/
