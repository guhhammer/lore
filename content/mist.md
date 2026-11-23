# mist

MIST - macOS Installer Super Tool. Automatically download macOS Firmwares/Installers.

## Usage
- `mist list firmware` : list all available macOS Firmwares for Apple Silicon Macs
- `mist list installer` : list all available macOS Installers for Intel Macs, including Universal Installers for macOS Big Sur and later
- `mist list installer --compatible` : list all macOS Installers that are compatible with this Mac, including Universal Installers for macOS Big Sur and later
- `mist list installer --include-betas` : list all available macOS Installers for Intel Macs, including betas, also including Universal Installers for macOS Big Sur and later
- `mist list installer --latest "macOS Sonoma"` : list only the latest macOS Sonoma Installer for Intel Macs, including Universal Installers for macOS Big Sur and later
- `mist list installer --export "/path/to/export.csv"` : list and export macOS Installers to a CSV file
- `mist download firmware "macOS Sonoma" --firmware-name "Install %NAME% %VERSION%-%BUILD%.ipsw"` : download the latest macOS Sonoma Firmware for Apple Silicon Macs, with a custom name
- `mist download installer "13.5.2" application` : download a specific macOS Installer version for Intel Macs, including Universal Installers for macOS Big Sur and later

## Categories
network, hardware, macos

## Links
- More information: https://github.com/ninxsoft/mist-cli
- tldr page: https://tldr.inbrowser.app/pages/osx/mist
