# ibmcloud api

IBM Cloud-API-Endpunkt festlegen oder anzeigen.

## Usage
- `ibmcloud api` : gib den aktuellen API-Endpunkt aus
- `ibmcloud api cloud.ibm.com` : lege den aktuellen API-Endpunkt als cloud.ibm.com fest
- `ibmcloud api private.cloud.ibm.com` : setze den privaten Endpunkt
- `ibmcloud api private.cloud.ibm.com --vpc` : benutze eine VPC-Verbindung für einen privaten API-Endpunkt
- `ibmcloud api https://cloud.ibm.com --skip-ssl-validation` : umgehe SSL Validierung für HTTPS Anfragen
- `ibmcloud api --unset` : entferne den aktuellen API-Endpunkt

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_api
