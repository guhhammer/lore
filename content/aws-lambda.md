# aws lambda

Use AWS Lambda, a compute service for running code without provisioning or managing servers.

## Usage
- `aws lambda invoke --function-name name path/to/response.json` : run a function
- `aws lambda invoke --function-name name --payload json path/to/response.json` : run a function with an input payload in JSON format
- `aws lambda list-functions` : list functions
- `aws lambda get-function-configuration --function-name name` : display the configuration of a function
- `aws lambda list-aliases --function-name name` : list function aliases
- `aws lambda get-function-concurrency --function-name name` : display the reserved concurrency configuration for a function
- `aws lambda get-policy --function-name name` : list which AWS services can invoke the function

## Categories
cloud, system, text

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lambda/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-lambda
