# iewzd

Internet Explorer Setup Wizard: Programmatically configure installation of Internet Explorer. Not to be confused with `iesetup`, which also bundles a customized version of Internet Explorer and `iewzd`. Note: You may need to replace the `iewzd` command with your desired IE version and executable, such as `Ie11wzd.exe`, `ie10_setup.exe`, or `IE9-Windows6.0-x86-en-us.exe`.

## Usage
- `tldr -p windows iesetup` : view the documentation for `iesetup`
- `iewzd` : run the Internet Explorer Wizard
- `iewzd /norestart|/forcerestart` : force or do not restart the computer after installing Internet Explorer
- `iewzd /no-default` : do not set Internet Explorer as the default browser
- `iewzd /update-no` : do not check for Internet Explorer updates
- `iewzd /passive|/quiet` : run the Wizard in passive (no interaction) or quiet (no interaction and output) mode
- `iewzd /log path\to\file` : save the installation log to a file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/iewzd
