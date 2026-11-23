# snowsql

SnowSQL client for Snowflake's Data Cloud.

## Usage
- `snowsql --accountname account --username username --dbname database --schemaname schema` : connect to a specific instance at <https://account.snowflakecomputing.com> (password can be provided in prompt or configuration file)
- `snowsql --config path/to/configuration_file` : connect to an instance specified by a specific configuration file (defaults to `~/.snowsql/config`)
- `snowsql --mfa-passcode token` : connect to the default instance using a token for multi-factor authentication
- `snowsql --query 'query'` : execute a single SQL query or SnowSQL command on the default connection (useful in shell scripts)
- `snowsql --filename path/to/file.sql` : execute commands from a specific file on the default connection

## Categories
databases, files, users

## Links
- More information: https://docs.snowflake.com/en/user-guide/snowsql
- tldr page: https://tldr.inbrowser.app/pages/common/snowsql
