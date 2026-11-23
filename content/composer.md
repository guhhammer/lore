# composer

A package-based dependency manager for PHP projects.

## Usage
- `composer init` : interactively create a `composer.json` file
- `composer require user/package` : add a package as a dependency for this project, adding an entry to `composer.json`
- `composer install` : install all the dependencies in this project's `composer.json` and create `composer.lock`
- `composer remove user/package` : uninstall a package from this project, removing it as a dependency from `composer.json` and `composer.lock`
- `composer update` : update all the dependencies in this project's `composer.json` and note new versions in `composer.lock` file
- `composer update --lock` : update only `composer.lock` after updating `composer.json` manually
- `composer why-not user/package` : learn more about why a dependency can't be installed
- `composer self-update` : update composer to its latest version

## Categories
packages, text, development

## Links
- More information: https://getcomposer.org/doc/03-cli.md
- tldr page: https://tldr.inbrowser.app/pages/common/composer
