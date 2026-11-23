# gcloud logging logs list

List logs in a Google Cloud project. Useful for identifying available logs for monitoring and analysis.

## Usage
- `gcloud logging logs list` : list all logs in the current project
- `gcloud logging logs list --bucket=bucket_id --location=location` : list all logs for a specific log bucket and location
- `gcloud logging logs list --bucket=bucket_id --location=location --view=view_id` : list all logs for a specific view in a log bucket
- `gcloud logging logs list --filter="expression"` : list logs with a filter expression
- `gcloud logging logs list --limit=number` : list a specified number of logs
- `gcloud logging logs list --sort-by="field_name"` : list logs sorted by a specific field in ascending or descending order (`~` for descending)
- `gcloud logging logs list --sort-by="field1,~field2"` : list logs sorted by multiple fields
- `gcloud logging logs list --verbosity=debug` : list logs with verbose output, showing additional details

## Categories
cloud, development

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/logging/logs/list
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-logging-logs-list
