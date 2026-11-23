# qmv

Move files and directories using the default text editor to define the filenames.

## Usage
- `qmv source_file` : move a single file (open an editor with the source filename on the left and the target filename on the right)
- `qmv *.jpg` : move multiple JPEG files
- `qmv -d path/to/directory1 path/to/directory2 path/to/directory3 ...` : move multiple directories
- `qmv -R path/to/directory` : move all files and directories inside a directory
- `qmv -o swap *.jpg` : move files, but swap the positions of the source and the target filenames in the editor
- `qmv -f do .` : rename all files and folders in the current directory, but show only target filenames in the editor (you can think of it as a kind of simple mode)

## Categories
files, editors, text

## Links
- More information: https://manned.org/qmv
- tldr page: https://tldr.inbrowser.app/pages/common/qmv
