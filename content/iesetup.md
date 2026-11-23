# iesetup

Build customized Internet Explorer Wizard (`iewzd`) installer. Note: You may need to replace the `iesetup` command with your desired IEAK version and executable, such as `Ie11setup` for IE 11. Part of Internet Explorer Administration Kit (IEAK).

## Usage
- `tldr -p windows iewzd` : view the documentation for `iewzd`
- `iesetup /c:"path\to\iewzd.exe"` : build a custom Internet Explorer Wizard executable, specifying the location of `iewzd` executable
- `iesetup /c:"path\to\iewzd.exe iewzd_args"` : apply custom `iewzd` program arguments into the custom installer
- `iesetup /q:a|u /c:"path\to\iewzd.exe iewzd_args"` : run the build process in a quiet mode, assuming the user account invoking this command as an administrator or regular user
- `iesetup /r:a|n|s /c:"path\to\iewzd.exe iewzd_args"` : specify the restart behavior after a successful installation (ask, never, or always)

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
development, files, users, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/iesetup
