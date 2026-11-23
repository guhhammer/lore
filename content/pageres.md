# pageres

Capture screenshots of websites in various resolutions.

## Usage
- `pageres https://example.com/ https://example2.com/ 1366x768 1600x900` : take multiple screenshots of multiple URLs at different resolutions
- `pageres [ https://example.com/ 1366x768 --no-crop ] [ https://example2.com/ 1024x768 ] -c` : provide specific options for a URL, overriding global options
- `pageres https://example.com/ 1024x768 --filename='<%= date %> - <%= url %>'` : provide a custom filename template
- `pageres https://example.com/ 1366x768 --selector='.page-header'` : capture a specific element on a page
- `pageres https://example.com/ 1366x768 --hide='.page-header'` : hide a specific element
- `pageres path/to/local_file.html 1366x768` : capture a screenshot of a local file

## Categories
network

## Links
- More information: https://github.com/sindresorhus/pageres-cli
- tldr page: https://tldr.inbrowser.app/pages/common/pageres
