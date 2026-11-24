# radare2

A set of reverse engineering tools.

## Usage
- `radare2 -nw path/to/binary` : open a file in write mode without parsing the file format headers
- `radare2 -d path/to/binary` : debug a program
- `radare2 -i path/to/script.r2 path/to/binary` : run a script before entering the interactive CLI
- `radare2_command?` : [Interactive] Display help text for any command
- `!shell_command` : [Interactive] Run a shell command from the interactive CLI
- `pr >path/to/file.bin` : [Interactive] Dump raw bytes of current block to a file

## Categories
files, development

## Links
- More information: https://book.rada.re/first_steps/commandline_flags.html
- tldr page: https://tldr.inbrowser.app/pages/common/radare2
- Wikipedia: https://en.wikipedia.org/wiki/Radare2
