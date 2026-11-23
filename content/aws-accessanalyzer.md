# aws accessanalyzer

Analyze and review resource policies to identify potential security risks.

## Usage
- `aws accessanalyzer create-analyzer --analyzer-name analyzer_name --type type --tags tags` : create a new Access Analyzer
- `aws accessanalyzer delete-analyzer --analyzer-arn analyzer_arn` : delete an existing Access Analyzer
- `aws accessanalyzer get-analyzer --analyzer-arn analyzer_arn` : get details of a specific Access Analyzer
- `aws accessanalyzer list-analyzers` : list all Access Analyzers
- `aws accessanalyzer update-analyzer --analyzer-arn analyzer_arn --tags new_tags` : update settings of an Access Analyzer
- `aws accessanalyzer create-archive-rule --analyzer-arn analyzer_arn --rule-name rule_name --filter filter` : create a new Access Analyzer archive rule
- `aws accessanalyzer delete-archive-rule --analyzer-arn analyzer_arn --rule-name rule_name` : delete an Access Analyzer archive rule
- `aws accessanalyzer list-archive-rules --analyzer-arn analyzer_arn` : list all Access Analyzer archive rules

## Categories
cloud, archives

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/accessanalyzer/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-accessanalyzer
