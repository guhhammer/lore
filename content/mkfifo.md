# mkfifo

Make named pipes, also known as First In First Out (FIFO).

## Usage
- `mkfifo path/to/pipe` : create a named pipe at a given path
- `echo "Hello World" >path/to/pipe &` : send data through a named pipe and send the command to the background
- `cat path/to/pipe` : receive data through a named pipe
- `mkfifo path/to/pipe; script -f path/to/pipe` : share your terminal session in real-time

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/mkfifo
