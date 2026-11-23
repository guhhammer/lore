# elasticsearch-croneval

Validate and evaluate a `cron` expression. This command helps verify that `cron` expressions are valid for use with Elasticsearch and produce the expected results.

## Usage
- `elasticsearch-croneval "cron_expression"` : evaluate a `cron` expression and display the next 10 trigger times (default behavior)
- `elasticsearch-croneval "cron_expression" -c integer` : evaluate a `cron` expression and display a specific number of future trigger times
- `elasticsearch-croneval "invalid_cron_expression" -d` : display detailed information (including stack trace) for an invalid `cron` expression
- `elasticsearch-croneval "cron_expression" -s` : display minimal output (silent mode)
- `elasticsearch-croneval "cron_expression" -v` : display verbose output

## Categories
system, desktop

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-croneval
