# less-than

Redirect data to `stdin`.

## Usage
- `command <path/to/file.txt` : redirect a file to `stdin` (achieves the same effect as `cat file.txt |`)
- `command <<EOF <Enter> multiline_text <Enter> EOF` : create a here document and pass that into `stdin` (requires a multiline command)
- `command <<<string` : create a here string and pass that into `stdin` (achieves the same effect as `echo string |`)
- `command <path/to/file.txt >path/to/file2.txt` : process data from a file and write the output to another file
- `cat <<EOF >path/to/file.txt <Enter> multiline_data <Enter> EOF` : write a here document into a file
- `cat <<-EOF >path/to/file.txt <Enter> multiline_data <Enter> EOF` : disregard leading tabs (good for scripts with indentation but does not work for spaces)
- `diff <(command1) <(command2)` : pass command output to a program as a file descriptor (Note: unlike the rest of these, this replaces the argument in-place with a file path like `/dev/fd/63`)
- `exec 3<path/to/file` : open a persistent file descriptor

## Categories
files, text

## Links
- More information: https://gnu.org/software/bash/manual/bash.html#Redirecting-Input
- tldr page: https://tldr.inbrowser.app/pages/common/less-than
