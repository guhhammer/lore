# docker image save

Exportiere eines oder mehrere Docker Images in ein Archiv.

## Usage
- `docker save image:tag >pfad/zu/datei.tar` : speichere ein Image über die Standardausgabe in ein `.tar`-Archiv
- `docker save -o pfad/zu/datei.tar image:tag` : speichere ein Image in ein `.tar`-Archiv
- `docker save -o pfad/zu/datei.tar image_name` : speichere alle Tags eines Images
- `docker save -o pfad/zu/datei.tar image_name:tag1 image_name:tag2 ...` : speichere nur bestimmte Tags eines Images

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
