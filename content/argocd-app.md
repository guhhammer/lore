# argocd app

Interface to manage applications by Argo CD.

## Usage
- `argocd app list --output json|yaml|wide` : list applications
- `argocd app get app_name --output json|yaml|wide` : get application details
- `argocd app create app_name --repo git_repo_url --path path/to/repo --dest-server https://kubernetes.default.svc --dest-namespace ns` : deploy application internally (to the same cluster that Argo CD is running in)
- `argocd app delete app_name` : delete an application
- `argocd app set app_name --sync-policy auto --auto-prune --self-heal` : enable application auto-sync
- `argocd app sync app_name --dry-run --prune` : simulate app synchronization, including pruning, without affecting the cluster
- `argocd app history app_name --output wide|id` : show application deployment history
- `argocd app rollback app_name history_id --prune` : roll back application to a previous deployed version by history ID (deleting unexpected resources)

## Categories
network, text, containers

## Links
- More information: https://argo-cd.readthedocs.io/en/stable/user-guide/commands/argocd_app/
- tldr page: https://tldr.inbrowser.app/pages/common/argocd-app
