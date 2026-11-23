# ibmcloud api

Set or view the IBM Cloud API endpoint.

## Usage
- `ibmcloud api` : view the current API endpoint
- `ibmcloud api cloud.ibm.com` : set the API endpoint to `cloud.ibm.com`
- `ibmcloud api private.cloud.ibm.com` : set a private API endpoint
- `ibmcloud api private.cloud.ibm.com --vpc` : use a VPC connection for a private endpoint
- `ibmcloud api https://cloud.ibm.com --skip-ssl-validation` : bypass SSL validation of HTTP requests
- `ibmcloud api --unset` : remove the API endpoint setting

## Categories
cloud, network

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_api
- tldr page: https://tldr.inbrowser.app/pages/common/ibmcloud-api
