# dvc config

Low level command to manage custom configuration options for dvc repositories. These configurations can be on project, local, global, or system level.

## Usage
- `dvc config core.remote` : get the name of the default remote
- `dvc config core.remote remote_name` : set the project's default remote
- `dvc config -u core.remote` : unset the project's default remote
- `dvc config key` : get the configuration value for a specified key for the current project
- `dvc config key value` : set the configuration value for a key on a project level
- `dvc config -u key` : unset a project level configuration value for a given key
- `dvc config --local|global|system key value` : set a local, global, or system level configuration value

## Categories
development

## Links
- More information: https://doc.dvc.org/command-reference/config
- tldr page: https://tldr.inbrowser.app/pages/common/dvc-config
