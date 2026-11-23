# gcloud

Das offizielle CLI-Tool für die Google Cloud Platform.

## Usage
- `gcloud config list` : liste alle Eigenschaften der aktiven Konfiguration auf
- `gcloud auth login` : mit einem Google-Konto anmelden
- `gcloud config set project Projektname` : lege das aktive Projekt fest
- `gcloud compute ssh Benutzer@Instanz` : stelle eine SSH-Verbindung zu einer virtuellen Maschineninstanz her
- `gcloud compute instances list` : zeige alle Google Compute Engine-Instanzen in einem Projekt an. Standardmäßig werden Instanzen aus allen Zonen aufgelistet
- `gcloud container clusters get-credentials Clustername` : aktualisiere eine kubeconfig-Datei mit den entsprechenden Anmeldeinformationen, um kubectl auf einen bestimmten Cluster in Google Kubernetes Engine auszurichten
- `gcloud components update` : aktualisiere alle gcloud CLI-Komponenten
- `gcloud help Befehl` : zeige Hilfe für einen bestimmten Befehl an

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
