# prowler m365

Assess Microsoft 365 (M365) security configurations and best practices.

## Usage
- `prowler m365 --env-auth` : run Prowler with combined service principal and user credentials
- `prowler m365 --sp-env-auth` : authenticate using a service principal
- `prowler m365 --az-cli-auth` : authenticate using the Azure CLI
- `prowler m365 --browser-auth --tenant-id "XXXXXXXX"` : authenticate using a browser and specify the tenant ID
- `prowler m365 -c etcd_enm365_onedrive_sharing_enabledcryption` : run a specific Microsoft 365 check
- `prowler m365 -e m365_onedrive_sharing_enabled` : exclude specific checks

## See also
- prowler aws
- prowler azure
- prowler gcp
- prowler kubernetes
- prowler github

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler-m365
