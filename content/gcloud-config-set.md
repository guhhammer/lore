# gcloud config set

Set a property in the Google Cloud CLI configuration. Properties control various aspects of Google Cloud CLI behavior.

## Usage
- `gcloud config set project project_id` : set the project property in the core section
- `gcloud config set compute/zone zone_name` : set the compute zone for future operations
- `gcloud config set disable_prompts true` : disable prompting to make gcloud suitable for scripting
- `gcloud config list` : view the list of properties currently in use
- `gcloud config unset property_name` : unset a previously set property
- `gcloud config configurations create configuration_name` : create a new configuration profile
- `gcloud config configurations activate configuration_name` : switch between different configuration profiles

## Categories
cloud

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/config/set
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-config-set
