# gyb

Locally back up Gmail messages using Gmail's API over HTTPS.

## Usage
- `gyb --email email@gmail.com --action estimate` : estimate the number and the size of all emails on your Gmail account
- `gyb --email email@gmail.com --action backup --local-folder path/to/directory` : back up a Gmail account to a specific directory
- `gyb --email email@gmail.com --search "is:important OR is:starred"` : back up only important or starred emails from a Gmail account to the default local folder
- `gyb --email email@gmail.com --action restore --local-folder path/to/directory` : restore from a local folder to a Gmail account

## Categories
files, network, users

## Links
- More information: https://github.com/GAM-team/got-your-back
- tldr page: https://tldr.inbrowser.app/pages/common/gyb
