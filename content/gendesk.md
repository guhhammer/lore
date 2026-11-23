# gendesk

Specify the command to generate a `.desktop` file and a download icon with minimal information.

## Usage
- `gendesk -n --name "app" --exec "/path/to/app" --icon "/path/to/icon.png" --comment "This is application"` : create a `.desktop` file named `app`
- `gendesk -q -f -n --name "app" --exec "/path/to/app" --icon "/path/to/icon.png" --comment "This is application"` : create a `.desktop` file named `app`, do not display any output, and overwrite it if it exists
- `gendesk -h` : display help

## Categories
files, desktop, network

## Links
- More information: https://manned.org/gendesk
- tldr page: https://tldr.inbrowser.app/pages/common/gendesk
