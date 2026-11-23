# ng config

Use JSON path notation (camelCase) to edit workspace or project configurations, such as build options.

## Usage
- `ng config` : display all configuration values
- `ng config projects.project_name.prefix` : get a specific configuration value
- `ng config projects.project_name.prefix value` : set a configuration value
- `ng config cli.analytics disabled -g` : disable CLI analytics globally
- `ng config projects.project_name.prefix value -g` : set a global config value (caution: this affects all Angular projects)

## Categories
development, files, text

## Links
- More information: https://angular.dev/cli/config
- tldr page: https://tldr.inbrowser.app/pages/common/ng-config
