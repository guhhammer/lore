# jhat

Java heap analysis tool.

## Usage
- `jhat dump_file.bin` : analyze a heap dump (from `jmap`), view via HTTP on port 7000
- `jhat -p port dump_file.bin` : analyze a heap dump, specifying an alternate port for the HTTP server
- `jhat -J-mx8G dump_file.bin` : analyze a dump letting `jhat` use up to 8 GB RAM (2-4x dump size recommended)

## Categories
network

## Links
- More information: https://docs.oracle.com/javase/8/docs/technotes/tools/unix/jhat.html
- tldr page: https://tldr.inbrowser.app/pages/common/jhat
