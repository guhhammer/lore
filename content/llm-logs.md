# llm logs

Explore prompts and responses logged to an SQLite database by the `llm` command.

## Usage
- `llm logs list` : show the most recent logged prompts and their responses (3 by default)
- `llm logs list -n 10` : show the n most recent entries
- `llm logs list -q query` : search logged prompts and responses for a query string
- `llm logs status` : show the current status of database logging
- `llm logs off` : turn off logging for all prompts
- `llm logs on` : turn logging back on
- `llm logs path` : print the path to the `logs.db` database file
- `llm logs backup path/to/backup.db` : back up the logs database to a file

## Categories
databases, files

## Links
- More information: https://llm.datasette.io/en/stable/help.html#llm-logs-help
- tldr page: https://tldr.inbrowser.app/pages/common/llm-logs
