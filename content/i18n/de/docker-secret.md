# docker secret

Verwalte Secrets für Docker Swarm.

## Usage
- `befehl | docker secret create secret_name -` : erstelle ein neues Secret über die Standardeingabe
- `docker secret create secret_name pfad/zu/datei` : erstelle ein neues Secret aus einer Datei
- `docker secret ls` : liste alle Secrets auf
- `docker secret inspect --pretty secret_name1 secret_name2 ...` : zeige detaillierte Informationen zu einem oder mehreren Secrets in einem menschenlesbaren Format
- `docker secret rm secret_name1 secret_name2 ...` : lösche eines oder mehrere Secrets

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
