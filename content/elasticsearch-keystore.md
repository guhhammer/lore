# elasticsearch-keystore

Manage secure settings (e.g., passwords, tokens, and credentials) used by Elasticsearch.

## Usage
- `elasticsearch-keystore create` : create a new keystore (not password-protected)
- `elasticsearch-keystore create -p` : create a new password-protected keystore
- `elasticsearch-keystore add setting_name` : add a setting interactively
- `echo "setting_value" | elasticsearch-keystore add --stdin setting_name` : add a setting from `stdin`
- `elasticsearch-keystore remove setting_name` : remove a setting from the keystore
- `elasticsearch-keystore passwd` : change the keystore password
- `elasticsearch-keystore list` : list all settings stored in the keystore
- `elasticsearch-keystore upgrade` : upgrade the keystore format (after an Elasticsearch upgrade)

## Categories
packages, security, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-keystore
