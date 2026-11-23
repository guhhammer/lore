# bq

A Python-based tool for BigQuery, Google Cloud's fully managed and completely serverless enterprise data warehouse.

## Usage
- `bq query --nouse_legacy_sql --dry_run 'SELECT COUNT(*) FROM DATASET_NAME.TABLE_NAME'` : simulate a query against a BigQuery table using standard SQL, to estimate the number of bytes it would read
- `bq query --use_legacy_sql=false --parameter='ts_value:TIMESTAMP:2016-12-07 08:00:00' 'SELECT TIMESTAMP_ADD(@ts_value, INTERVAL 1 HOUR)'` : run a parameterized query
- `bq mk --location=US dataset_name.table_name` : create a new dataset or table in the US location
- `bq ls --filter labels.key:value --max_results integer --format=prettyjson --project_id project_id` : list all datasets in a project
- `bq load --location location --source_format CSV|JSON|PARQUET|AVRO dataset.table path/to/source` : batch load data from a specific file in formats such as CSV, JSON, Parquet, and Avro to a table
- `bq cp dataset.OLD_TABLE dataset.new_table` : copy one table to another
- `bq help` : display help

## Categories
cloud, databases, text

## Links
- More information: https://docs.cloud.google.com/bigquery/docs/reference/bq-cli-reference
- tldr page: https://tldr.inbrowser.app/pages/common/bq
