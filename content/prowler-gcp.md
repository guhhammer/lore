# prowler gcp

Assess Google Cloud Platform (GCP) security best practices, audits, and compliance checks.

## Usage
- `prowler gcp` : run the default set of checks on all accessible GCP projects using default user credentials
- `prowler gcp --credentials-file path/to/credentials.json` : authenticate using a service account credentials file
- `prowler gcp --project-ids project_id1 project_id2 ...` : scan specific GCP projects by ID
- `prowler gcp -s iam|compute|...` : run checks for selected GCP services
- `prowler gcp -c gcp_storage_bucket_logging_enabled` : run a specific GCP check
- `prowler gcp -e gcp_storage_bucket_logging_enabled --exclude-services iam|compute|...` : exclude specific checks or services

## See also
- prowler aws
- prowler azure
- prowler kubernetes
- prowler m365
- prowler github

## Categories
cloud, system, files

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler-gcp
