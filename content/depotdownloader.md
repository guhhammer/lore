# depotdownloader

Content/depot downloader for Steam.

## Usage
- `depotdownloader -app 108600` : download an app
- `depotdownloader -app 108600 -depot 108603 -dir path/to/directory` : download a specific depot into a custom output directory
- `depotdownloader -app 108600 -depot 108603 -username "gabecube"` : download using a Steam account
- `depotdownloader -app 108600 -depot 108603 -username "gabecube" -remember-password` : download a depot and remember the password for future downloads
- `depotdownloader -app 346110 -depot 346111 -manifest 6154025194991279746` : download a specific depot manifest
- `depotdownloader -app 108600 -depot 108603 -branch "unstable"` : download from a specific branch
- `depotdownloader -app 108600 -depot 108603 -manifest-only` : download only the internal manifest excluding content
- `depotdownloader -app 108600 -pubfile 2503622437` : download workshop content using the pubfile/workshop id

## Categories
network, files, users

## Links
- More information: https://github.com/SteamRE/DepotDownloader
- tldr page: https://tldr.inbrowser.app/pages/common/depotdownloader
