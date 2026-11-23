# dex

DesktopEntry Execution is a program to generate and execute DesktopEntry files of the Application type.

## Usage
- `dex -a` : execute all programs in the autostart folders
- `dex -a -s path/to/directory1:path/to/directory2:path/to/directory3:` : execute all programs in the specified folders
- `dex -a -e GNOME` : preview the programs would be executed in a GNOME specific autostart
- `dex -a -d` : simulate a regular autostart, showing the programs that would be executed
- `dex -p Name path/to/file.desktop` : preview the value of the DesktopEntry property `Name`
- `dex -c path/to/file.desktop` : create a DesktopEntry for a program in the current directory
- `dex --term terminal path/to/file.desktop` : execute a single program (with `Terminal=true` in the `.desktop` file) in the given terminal

## Categories
files, desktop, shell

## Links
- More information: https://github.com/jceb/dex#dex
- tldr page: https://tldr.inbrowser.app/pages/linux/dex
