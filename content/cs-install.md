# cs install

Install an application in the installation directory configured when installing `cs` (to enable the binary to be loaded add to your `.bash_profile` the `$ eval "$(cs install --env)"` command).

## Usage
- `cs install application_name` : install a specific application
- `cs install application_name:application_version` : install a specific version of an application
- `cs search application_partial_name` : search an application by a specific name
- `cs update application_name` : update a specific application if available
- `cs update` : update all the installed applications
- `cs uninstall application_name` : uninstall a specific application
- `cs list` : list all installed applications
- `application_name -Jjava_option_name1=value1 -Jjava_option_name2=value2 ...` : pass specific Java options to an installed application

## Categories
packages, files

## Links
- More information: https://get-coursier.io/docs/cli-install
- tldr page: https://tldr.inbrowser.app/pages/common/cs-install
