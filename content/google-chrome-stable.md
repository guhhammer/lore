# google-chrome-stable

This command is an alias of `chromium`.

## Usage
- `chromium https://example.com|path/to/file.html` : open a specific URL or file
- `chromium --incognito example.com` : open in incognito mode
- `chromium --new-window example.com` : open in a new window
- `chromium --app=https://example.com` : open in application mode (without toolbars, URL bar, buttons, etc.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : use a proxy server
- `chromium --user-data-dir=path/to/directory` : open with a custom profile directory
- `chromium --user-data-dir=path/to/directory --disable-web-security` : open without CORS validation (useful to test an API)
- `chromium --auto-open-devtools-for-tabs` : open with a DevTools window for each tab opened

## Categories
files, network, desktop

## Links
- More information: https://www.google.com/chrome/
- tldr page: https://tldr.inbrowser.app/pages/linux/google-chrome-stable
