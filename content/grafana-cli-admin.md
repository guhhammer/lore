# grafana cli admin

Manage Grafana administrator commands.

## Usage
- `grafana cli admin reset-admin-password new_password` : reset the admin password
- `grafana cli admin reset-admin-password --user-id user_id new_password` : reset the password for a specific user
- `grafana cli --homepath path/to/grafana admin reset-admin-password new_password` : reset the password using a specific Grafana home directory
- `grafana cli admin -h` : display help
- `grafana cli admin -v` : display version

## Categories
users

## Links
- More information: https://grafana.com/docs/grafana/latest/administration/cli/#admin-commands
- tldr page: https://tldr.inbrowser.app/pages/common/grafana-cli-admin
