# microsoft-edge

Modern web browser developed by Microsoft based on the Chromium web browser developed by Google. This command is available instead as `msedge` for Windows. Note: Additional command arguments from `chromium` may also be usable to control Microsoft Edge.

## Usage
- `microsoft-edge https://example.com|path/to/file.html` : open a specific URL or file
- `microsoft-edge --inprivate example.com` : open in InPrivate mode
- `microsoft-edge --new-window example.com` : open in a new window
- `microsoft-edge --app=https://example.com` : open in application mode (without toolbars, URL bar, buttons, etc.)
- `microsoft-edge --proxy-server="socks5://hostname:66" example.com` : use a proxy server
- `microsoft-edge --user-data-dir=path/to/directory` : open with a custom profile directory
- `microsoft-edge --user-data-dir=path/to/directory --disable-web-security` : open without CORS validation (useful to test an API)
- `microsoft-edge --auto-open-devtools-for-tabs` : open with a DevTools window for each tab opened

## Categories
files, network, desktop

## Links
- More information: https://microsoft.com/edge
- tldr page: https://tldr.inbrowser.app/pages/common/microsoft-edge
