# mas

Command-line interface for the Mac App Store.

## Usage
- `mas signin "user@example.com"` : sign into the Mac App Store for the first time
- `mas list` : show all installed applications and their product identifiers
- `mas search "application" --price` : search for an application, displaying the price alongside the results
- `mas install numeric_product_id` : install or update an application using exact numeric id
- `mas lucky "search_term"` : install the first application that would be returned by the respective search
- `mas outdated` : list all outdated apps with pending updates
- `mas upgrade` : install all pending updates
- `mas upgrade "numeric_product_id"` : upgrade a specific application

## Categories
packages, network, text, macos

## Links
- More information: https://github.com/mas-cli/mas
- tldr page: https://tldr.inbrowser.app/pages/osx/mas
