# hledger-web

A web interface and API for `hledger`, a robust, friendly plain text accounting app.

## Usage
- `hledger-web` : start the web app, and a browser if possible, for local viewing and adding only
- `hledger-web -f path/to/file.journal --allow edit` : start the web app with a specific file, and allow editing of existing data
- `hledger-web --serve --host my.host.name --port 8000` : start just the web app, and accept incoming connections to the specified host and port
- `hledger-web --serve-api --host my.host.name --allow view` : start just the web app's JSON API, and allow only read access
- `hledger-web --value now --infer-market-prices` : show amounts converted to current market value in your base currency when known
- `hledger-web --info` : show the manual in Info format if possible
- `hledger-web -h` : display help

## Categories
network, text, files

## Links
- More information: https://hledger.org/hledger-web.html
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-web
