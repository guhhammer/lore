# argocd app

Interfaz de línea de comandos para gestionar aplicaciones por CD Argo.

## Usage
- `argocd app list --output json|yaml|wide` : lista aplicaciones
- `argocd app get nombre_de_la_aplicacion --output json|yaml|wide` : obtén los detalles de la aplicación
- `argocd app create nombre_de_la_aplicación --repo git_repo_url --path ruta/al/repo --dest-server https://kubernetes.default.svc --dest-namespace ns` : despliega la aplicación internamente (en el mismo clúster en el que se ejecuta Argo CD)
- `argocd app delete nombre_de_la_aplicación` : elimina una aplicación
- `argocd app set nombre_de_la_aplicacion --sync-policy auto --auto-prune --self-heal` : activa la sincronización automática de aplicaciones
- `argocd app sync nombre_de_la_aplicacion --dry-run --prune` : previsualiza la sincronización de aplicaciones sin afectar al clúster
- `argocd app history nombre_de_la_aplicacion --output wide|id` : muestra el historial de despliegue de aplicaciones
- `argocd app rollback nombre_de_la_aplicacion identificador_de_historial --prune` : retrocede la aplicación a una versión anterior desplegada por ID de historial (eliminando recursos inesperados)

## Links
- More information: https://argo-cd.readthedocs.io/en/stable/user-guide/commands/argocd_app/
