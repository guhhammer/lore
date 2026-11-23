# aws cur

Create, query, and delete AWS usage report definitions.

## Usage
- `aws cur put-report-definition --report-definition file://path/to/report_definition.json` : create an AWS cost and usage report definition from a JSON file
- `aws cur describe-report-definitions` : list usage report definitions defined for the logged in account
- `aws cur --region aws_region delete-report-definition --report-name report` : delete a usage report definition

## Categories
cloud, monitoring, databases

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-cur
