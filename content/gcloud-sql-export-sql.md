# gcloud sql export sql

Export data from a Cloud SQL instance to an SQL file in Google Cloud Storage. Useful for creating backups or migrating data.

## Usage
- `gcloud sql export sql instance gs://bucket_name/file_name` : export data from a specific Cloud SQL instance to a Google Cloud Storage bucket as an SQL dump file
- `gcloud sql export sql instance gs://bucket_name/file_name --async` : export data asynchronously, returning immediately without waiting for the operation to complete
- `gcloud sql export sql instance gs://bucket_name/file_name --database=database1,database2,...` : export data from specific databases within the Cloud SQL instance
- `gcloud sql export sql instance gs://bucket_name/file_name --database=database --table=table1,table2,...` : export specific tables from a specified database within the Cloud SQL instance
- `gcloud sql export sql instance gs://bucket_name/file_name --offload` : export data while offloading the operation to a temporary instance to reduce strain on the source instance
- `gcloud sql export sql instance gs://bucket_name/file_name.gz` : export data and compress the output with `gzip`

## Categories
databases, cloud, files

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/sql/export/sql
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-sql-export-sql
