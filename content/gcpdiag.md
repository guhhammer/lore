# gcpdiag

Google Cloud Platform troubleshooting and diagnostics tool. Run in a Docker container or in GCP Cloudshell.

## Usage
- `gcpdiag lint --project gcp_project_id` : run `gcpdiag` on your project, returning all rules
- `gcpdiag lint --project gcp_project_id --hide-ok` : hide rules that are ok
- `gcpdiag lint --project gcp_project_id --auth-key path/to/private_key` : authenticate using a service account private key file
- `gcpdiag lint --project gcp_project_id --within-days number` : search logs and metrics from a number of days back (default: 3 days)
- `gcpdiag lint --help` : display help

## Categories
cloud, containers, development

## Links
- More information: https://github.com/GoogleCloudPlatform/gcpdiag#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gcpdiag
