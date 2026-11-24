# wine

Run Windows executables on Unix-based systems.

## Usage
- `wine command` : run a specific program inside the `wine` environment
- `wine start command` : run a specific program in background
- `wine msiexec /i|x path/to/package.msi` : install/uninstall an MSI package
- `wine explorer|notepad|write` : run `File Explorer`, `Notepad`, or `WordPad`
- `wine regedit|control|taskmgr` : run `Registry Editor`, `Control Panel`, or `Task Manager`
- `wine explorer /desktop=shell,1920x1080` : open a virtual Wine desktop
- `wine winecfg` : run the configuration tool
- `wine reg add 'path\to\registry_key' /v Value_name /t REG_SZ|REG_BINARY|REG_DWORD|... /d 'data'` : add a registry key

## Categories
files, packages

## Links
- More information: https://gitlab.winehq.org/wine/wine/-/wikis/Commands
- tldr page: https://tldr.inbrowser.app/pages/linux/wine
- Wikipedia: https://en.wikipedia.org/wiki/Wine_(software)
