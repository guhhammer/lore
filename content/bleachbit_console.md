# bleachbit_console

Clean junk files on the filesystem.

## Usage
- `bleachbit_console.exe --gui` : start the graphical user interface (GUI) version of Bleachbit
- `bleachbit_console.exe --shred path\to\file` : shred a file
- `bleachbit_console.exe --list-cleaners` : list available cleaner options
- `bleachbit_console.exe --preview cleaner1.option1 cleaner2.* ...` : preview the files that will be deleted and other changes that will be made before actually performing the clean-up operation
- `bleachbit_console.exe --clean cleaner1.option1 cleaner2.* ...` : perform the clean-up operation and delete files
- `bleachbit_console.exe --preview --preset cleaner1.option1 cleaner2.* ...` : select the same options as in the GUI and add more to it

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, disks, files, windows

## Links
- More information: https://docs.bleachbit.org/doc/command-line-interface.html
- tldr page: https://tldr.inbrowser.app/pages/windows/bleachbit_console
