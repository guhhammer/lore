# electron-packager

Build Electron app executables for Windows, Linux, and macOS. Requires a valid `package.json` in the application directory.

## Usage
- `electron-packager "path/to/app" "app_name"` : package an application for the current architecture and platform
- `electron-packager "path/to/app" "app_name" --all` : package an application for all architectures and platforms
- `electron-packager "path/to/app" "app_name" --platform="linux" --arch="x64"` : package an application for 64-bit Linux
- `electron-packager "path/to/app" "app_name" --platform="darwin" --arch="arm64"` : package an application for ARM macOS

## Categories
files, packages, development

## Links
- More information: https://github.com/electron/packager
- tldr page: https://tldr.inbrowser.app/pages/common/electron-packager
