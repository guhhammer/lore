# gcloud

gcloud is the command line of Google Cloud. This page covers signing in, projects, Compute Engine, Cloud Storage, Cloud Run and Kubernetes.

## Tasks
### Sign in and choose a project
- `gcloud auth login` : sign in through the browser
- `gcloud auth application-default login` : give local programs credentials
- `gcloud config set project PROJECT_ID` : choose the default project
- `gcloud config list` : show the settings in use
- `gcloud projects list` : projects you can see
- `gcloud config set compute/region REGION` : choose a default region
- `gcloud auth list` : accounts signed in
### Servers (Compute Engine)
- `gcloud compute instances list` : virtual machines
- `gcloud compute ssh NAME --zone ZONE` : open a shell on one
- `gcloud compute instances stop NAME --zone ZONE` : stop it
- `gcloud compute instances start NAME --zone ZONE` : start it
- `gcloud compute firewall-rules list` : firewall rules
### Files (Cloud Storage)
- `gcloud storage ls` : list buckets
- `gcloud storage cp FILE gs://BUCKET/` : upload
- `gcloud storage cp gs://BUCKET/FILE .` : download
- `gcloud storage rsync DIR gs://BUCKET/PATH` : sync a folder
- `gcloud storage rm -r gs://BUCKET/PATH` : delete a path recursively (deletes it)
### Containers and apps
- `gcloud run deploy NAME --image IMAGE --region REGION` : deploy to Cloud Run
- `gcloud run services list` : Cloud Run services
- `gcloud container clusters get-credentials NAME --region REGION` : point kubectl at a GKE cluster
- `gcloud auth configure-docker REGION-docker.pkg.dev` : let Docker push to Artifact Registry
- `gcloud functions list` : Cloud Functions
### Logs and access
- `gcloud logging read "severity>=ERROR" --limit 20` : recent errors
- `gcloud iam service-accounts list` : service accounts
- `gcloud projects get-iam-policy PROJECT_ID` : who can do what
- `gcloud components update` : update gcloud (if installed from the archive)
### Output
- `gcloud ... --format=json` : JSON (also yaml, table, value)
- `gcloud ... --filter="name~NAME"` : keep only matching rows
- `gcloud help` : manual of a command, add after any command

## Notes
- Many commands need a project and a zone or region: set defaults with `gcloud config set`.
- A new project often needs the service's API turned on: `gcloud services enable NAME.googleapis.com`.
- `gsutil` is the older storage tool; `gcloud storage` is its replacement.
- Install from the official page in Links.

## See also
- aws
- az
- terraform
- kubectl

## Links
- Reference: https://cloud.google.com/sdk/gcloud/reference
- Install: https://cloud.google.com/sdk/docs/install

## Categories
cloud
