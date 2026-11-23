# security-checker

Check if a PHP application uses dependencies with known security vulnerabilities.

## Usage
- `security-checker security:check` : look for security issues in the project dependencies (based on the `composer.lock` file in the current directory)
- `security-checker security:check path/to/composer.lock` : use a specific `composer.lock` file
- `security-checker security:check --format=json` : return results as a JSON object

## Categories
files, packages

## Links
- More information: https://github.com/sensiolabs/security-checker
- tldr page: https://tldr.inbrowser.app/pages/common/security-checker
