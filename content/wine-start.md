# wine start

Launch a program or open a document in a Wine prefix. A reimplementation of the Windows `start` command.

## Usage
- `wine start path\to\program.exe` : launch a Windows program
- `wine start path\to\document.ext` : open a document with its associated program
- `wine start /unix path/to/program.exe` : launch a program from a Unix-style path
- `wine start /wait path\to\program.exe` : launch a program and wait for it to exit, returning its exit code
- `wine start /min|/max path\to\program.exe` : launch a program minimized or maximized
- `wine start /d path\to\directory path\to\program.exe` : launch a program from a specific working directory
- `wine start /b path\to\program.exe` : launch a program without opening a new console window
- `wine start /?` : display help

## Categories
files, documents

## Links
- More information: https://gitlab.winehq.org/wine/wine/-/wikis/Commands
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-start
