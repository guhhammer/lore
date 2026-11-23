# duti

Set default applications for document types and URL schemes on macOS.

## Usage
- `duti -s com.apple.Safari public.html all` : set Safari as the default handler for HTML documents
- `duti -s org.videolan.vlc m4v viewer` : set VLC as the default viewer for files with `.m4v` extensions
- `duti -s com.apple.Finder "ftp"` : set Finder as the default handler for the ftp:// URL scheme
- `duti -x ext` : display information about the default application for a given extension
- `duti -d uti` : display the default handler for a given UTI
- `duti -l uti` : display all handlers of a given UTI

## See also
- osascript

## Categories
network, desktop, documents, macos

## Links
- More information: https://github.com/moretension/duti
- tldr page: https://tldr.inbrowser.app/pages/osx/duti
