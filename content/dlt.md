# dlt

Create, inspect, and maintain data load tool (dlt) pipelines.

## Usage
- `dlt init source destination` : create a pipeline project in the current directory from a verified source or a template
- `dlt init --list-sources|--list-destinations` : list the available sources or destinations
- `dlt pipeline -l` : list local pipelines
- `dlt pipeline pipeline_name info` : display the state of a pipeline
- `dlt pipeline pipeline_name trace` : display the trace of the last run of a pipeline
- `dlt pipeline pipeline_name failed-jobs` : display the failed jobs of a pipeline with their error messages
- `dlt pipeline pipeline_name schema --format json|yaml|dbml|dot|mermaid` : display the default schema of a pipeline in a specific format
- `dlt pipeline pipeline_name drop resource1 resource2 ...` : drop tables of specific resources and reset their state

## Categories
desktop, monitoring, network

## Links
- More information: https://dlthub.com/docs/reference/command-line-interface
- tldr page: https://tldr.inbrowser.app/pages/common/dlt
