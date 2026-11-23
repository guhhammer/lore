# wp

The official interface to manage WordPress instances.

## Usage
- `wp --info` : print information about the operating system, shell, PHP, and WP-CLI (`wp`) installation
- `wp cli update` : update WP-CLI
- `wp core download --locale=locale` : download a fresh WordPress installation to current directory, optionally specifying the locale
- `wp config create --dbname=dbname --dbuser=dbuser --dbpass=dbpass` : create basic `wpconfig` file (assuming database on `localhost`)
- `wp plugin install plugin --activate` : install and activate a WordPress plugin
- `wp search-replace old_string new_string` : replace all instances of a string in the database
- `wp import path/to/file.xml` : import the contents of a WordPress Extended RSS (WXR) file

## Categories
network, files, system

## Links
- More information: https://developer.wordpress.org/cli/commands/
- tldr page: https://tldr.inbrowser.app/pages/common/wp
