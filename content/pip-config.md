# pip config

Manage local and global configuration for pip.

## Usage
- `pip config list` : list all configuration values
- `pip config debug` : show configuration files and their values
- `pip config set command.option value --global|--user|--site` : set the value for a command option
- `pip config get command.option --global|--user|--site` : get the value for a command option
- `pip config unset command.option --global|--user|--site` : unset the value for a command option
- `pip config edit --global|--user|--site` : edit the configuration file with the default editor
- `pip config edit --global|--user|--site --editor path/to/editor_binary` : edit the configuration file with a specific editor

## Categories
users, editors, files

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_config/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-config
- Wikipedia: https://en.wikipedia.org/wiki/Pip_(package_manager)
