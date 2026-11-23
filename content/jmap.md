# jmap

Java memory map tool.

## Usage
- `jmap java_pid` : print shared object mappings for a Java process (output like pmap)
- `jmap -heap filename.jar java_pid` : print heap summary information
- `jmap -histo java_pid` : print histogram of heap usage by type
- `jmap -dump:format=b,file=path/to/file java_pid` : dump contents of the heap into a binary file for analysis with jhat
- `jmap -dump:live,format=b,file=path/to/file java_pid` : dump live objects of the heap into a binary file for analysis with jhat

## Categories
files, monitoring, documents

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/jmap.html
- tldr page: https://tldr.inbrowser.app/pages/common/jmap
