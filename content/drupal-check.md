# drupal-check

Check Drupal PHP code for deprecations.

## Usage
- `drupal-check path/to/directory` : check the code in a specific directory for deprecations
- `drupal-check -e path/to/excluded_directory,path/to/excluded_files/*.php path/to/directory` : check the code excluding a comma-separated list of directories
- `drupal-check --no-progress path/to/directory` : don't show a progress bar
- `drupal-check -a path/to/directory` : perform static analysis to detect bad coding practices

## Categories
files

## Links
- More information: https://github.com/mglaman/drupal-check#usage
- tldr page: https://tldr.inbrowser.app/pages/common/drupal-check
