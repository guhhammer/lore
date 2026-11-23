# kubeseal

Client-side utility for encrypting Kubernetes secrets using the Bitnami Sealed Secrets controller. Creates SealedSecret resources that can be safely stored in version control. Requires a controller running in the cluster (e.g., installed via `kubectl apply --filename controller.yaml`).

## Usage
- `kubeseal <secret.yaml >sealedsecret.json` : encrypt a Kubernetes secret from a YAML file into a SealedSecret (default JSON output)
- `kubeseal <secret.yaml -o yaml|json --token my-bearer-token >sealedsecret.yaml` : encrypt a secret, outputting it in YAML or JSON format, using a bearer token for API authentication
- `kubeseal <secret.yaml --controller-namespace controller_namespace --controller-name controller_name >sealedsecret.yaml` : seal a secret using a specific controller namespace of sealed-secrets controller and name
- `kubeseal --raw --from-file path/to/secret.txt --name my-secret --scope strict|namespace-wide|cluster-wide >sealedsecret.yaml` : encrypt a raw secret value from a file with a specified name and scope
- `kubeseal --fetch-cert --username username --password password >cert.pem` : fetch the controller's public certificate for offline sealing with basic auth
- `kubeseal <secret.yaml --cert cert.pem >sealedsecret.yaml` : seal a secret offline using a fetched certificate
- `kubeseal <secret.yaml --merge-into sealedsecret.yaml` : merge a secret into an existing SealedSecret file in-place
- `kubeseal <sealedsecret.yaml --validate` : validate a SealedSecret without applying it

## Categories
text, containers, files

## Links
- More information: https://github.com/bitnami-labs/sealed-secrets
- tldr page: https://tldr.inbrowser.app/pages/common/kubeseal
