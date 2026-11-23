# gcloud config

Manage different configurations of `gcloud`.

## Usage
- `gcloud config set property value` : define a property (like compute/zone) for the current configuration
- `gcloud config get property` : fetch the value of a `gcloud` property
- `gcloud config list` : display all the properties for the current configuration
- `gcloud config configurations create configuration_name` : create a new configuration with a given name
- `gcloud config configurations list` : display a list of all available configurations
- `gcloud config configurations activate configuration_name` : switch to an existing configuration with a given name

## Categories
cloud

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/config
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-config
