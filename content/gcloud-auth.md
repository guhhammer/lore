# gcloud auth

Grant and revoke authorization to `gcloud` and manage credentials.

## Usage
- `gcloud auth login` : authorize Google Cloud access for the `gcloud` CLI with Google Cloud user credentials and set the current account as active
- `gcloud auth activate-service-account` : authorize Google Cloud access similar to `gcloud auth login` but with service account credentials
- `gcloud auth application-default` : manage Application Default Credentials (ADC) for Cloud Client Libraries
- `gcloud auth list` : display a list of Google Cloud accounts currently authenticated on your system
- `gcloud auth print-access-token` : display the current account's access token
- `gcloud auth revoke` : remove access credentials for an account

## Categories
cloud, users

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/auth
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-auth
