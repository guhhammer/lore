# gcloud

Det officielle CLI værktøj for Google Cloud Platform.

## Usage
- `gcloud config list` : list alle aktive konfigurationer
- `gcloud auth login` : login på en Google account
- `gcloud config set project projekt_navn` : sæt et GCP project som standard
- `gcloud compute ssh bruger@instans` : SSH ind til en virtuel maskine
- `gcloud compute instances list` : vis et overblik af alle Google Compute Engine instanser i et project. Instanser fra alle zoner er listet som standard
- `gcloud container clusters get-credentials cluster_navn` : opdater en kube-konfiguratonsfil med de korrekte credentials, der peger kubectl mod et spesifikt cluster i Google Kubernetes Engine
- `gcloud components update` : opdater all gcloud CLI komponenter
- `gcloud help kommando` : vis hjælp for en command

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
