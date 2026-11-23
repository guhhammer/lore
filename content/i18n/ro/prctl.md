# prctl

Obține sau setează controalele de resurse ale proceselor, sarcinilor și proiectelor care rulează.

## Usage
- `prctl pid` : examinează limitele și permisiunile procesului
- `prctl -P pid` : examinează limitele și permisiunile procesului într-un format prelucrabil automat
- `prctl -n process.max-file-descriptor pid` : obține o limită specifică pentru un proces care rulează

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
