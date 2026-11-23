# powercfg

Configure power settings and manage power schemes.

## Usage
- `powercfg /getactivescheme` : display the current power scheme
- `powercfg /L` : list all available power schemes
- `powercfg /setactive guid` : set the active power scheme by its GUID
- `powercfg /X monitor-timeout-ac minutes` : turn off the display after a specific number of minutes on AC power
- `powercfg /X monitor-timeout-dc minutes` : turn off the display after a specific number of minutes on battery power
- `powercfg /X standby-timeout-ac minutes` : put the system to sleep after a specific number of minutes on AC power
- `powercfg /sleepstudy /output path\to\report.html` : generate a system sleep diagnostics report and save it to a file
- `powercfg /energy /output path\to\report.html` : generate a full power efficiency report and save it to a file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/powercfg
