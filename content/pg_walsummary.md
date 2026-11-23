# pg_walsummary

Print contents of WAL summary files.

## Usage
- `pg_walsummary path/to/file` : convert a WAL summary file to text
- `pg_walsummary -i path/to/file` : print one line per individual modified block (rather than ranges)
- `pg_walsummary -q path/to/file` : suppress normal output (only errors)

## Categories
files, documents

## Links
- More information: https://www.postgresql.org/docs/current/app-pgwalsummary.html
- tldr page: https://tldr.inbrowser.app/pages/common/pg_walsummary
