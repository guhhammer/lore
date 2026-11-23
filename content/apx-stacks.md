# apx stacks

Manage stacks in `apx`. Note: User-created stack configurations are stored in `~/.local/share/apx/stacks`.

## Usage
- `apx stacks new` : interactively create a new stack configuration
- `apx stacks update name` : interactively update a stack configuration
- `apx stacks list` : list all available stack configurations
- `apx stacks rm --name string` : remove a specified stack configuration
- `apx stacks import --input path/to/stack.yml` : import a stack configuration
- `apx stacks export --name string --output path/to/output_file` : export the stack configuration (Note: The output flag is optional, it is exported to the current working directory by default)

## Categories
files, users

## Links
- More information: https://docs.vanillaos.org/docs/en/apx-manpage#stacks
- tldr page: https://tldr.inbrowser.app/pages/linux/apx-stacks
