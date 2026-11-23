# winemaker

Generate build files to port Windows source code to a Winelib application.

## Usage
- `winemaker path/to/source_directory` : generate build files for a source tree (assumes a graphical application)
- `winemaker --console path/to/source_directory` : treat the target as a console application
- `winemaker --dll path/to/source_directory` : treat the target as a DLL
- `winemaker --nosource-fix path/to/source_directory` : generate build files without modifying or renaming the original sources
- `winemaker --mfc path/to/source_directory` : process a project that uses MFC
- `winemaker --help` : display help

## Categories
development, files, network

## Links
- More information: https://manned.org/winemaker
- tldr page: https://tldr.inbrowser.app/pages/linux/winemaker
