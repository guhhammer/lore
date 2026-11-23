# pgbench

Run a benchmark test on PostgreSQL.

## Usage
- `pgbench --initialize --scale=50 database_name` : initialize a database with a scale factor of 50 times the default size
- `pgbench --client=10 --jobs=2 --transactions=10000 database_name` : benchmark a database with 10 clients, 2 worker threads, and 10,000 transactions per client

## Categories
monitoring, development

## Links
- More information: https://www.postgresql.org/docs/current/pgbench.html
- tldr page: https://tldr.inbrowser.app/pages/common/pgbench
