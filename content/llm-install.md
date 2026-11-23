# llm install

Install Python packages in the same environment as LLM, typically to add plugins.

## Usage
- `llm install plugin_name` : install a plugin from PyPI
- `llm install -U plugin_name` : upgrade a plugin to its latest version
- `llm install -e path/to/plugin_directory` : install a local plugin in editable mode
- `llm install --force-reinstall plugin_name` : reinstall a plugin even if it is already up to date
- `llm install --pre plugin_name` : include pre-release versions when installing a plugin

## Categories
packages

## Links
- More information: https://llm.datasette.io/en/stable/help.html#llm-install-help
- tldr page: https://tldr.inbrowser.app/pages/common/llm-install
