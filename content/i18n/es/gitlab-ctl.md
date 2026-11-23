# gitlab-ctl

Gestiona el ómnibus de GitLab.

## Usage
- `sudo gitlab-ctl status` : muestra el estado de cada servicio
- `sudo gitlab-ctl status nginx` : muestra el estado de un servicio específico
- `sudo gitlab-ctl restart` : reinicia todos los servicios
- `sudo gitlab-ctl restart nginx` : reinicia un servicio específico
- `sudo gitlab-ctl tail` : muestra los registros de cada servicio y sigue leyendo hasta que se pulse `<Ctrl c>`
- `sudo gitlab-ctl tail nginx` : muestra los registros de un servicio específico
- `sudo gitlab-ctl kill nginx` : envia la señal `SIGKILL` a un servicio específico
- `sudo gitlab-ctl reconfigure` : reconfigura la aplicación

## Links
- More information: https://docs.gitlab.com/omnibus/maintenance/
