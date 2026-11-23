# cmctl

Manage cert-manager resources inside your cluster. Check cert signing status, approve/deny requests, and issue new certificate requests.

## Usage
- `cmctl check api` : check if the cert-manager API is ready
- `cmctl status certificate cert_name` : check the status of a certificate
- `cmctl create certificaterequest my-cr --from-certificate-file cert.yaml` : create a new certificate request based on an existing certificate
- `cmctl create certificaterequest my-cr --from-certificate-file cert.yaml --fetch-certificate --timeout 20m` : create a new certificate request, fetch the signed certificate, and set a maximum wait time

## Categories
security, containers

## Links
- More information: https://cert-manager.io/docs/reference/cmctl/
- tldr page: https://tldr.inbrowser.app/pages/common/cmctl
