# compress-archive

A cmdlet in PowerShell is used to create compressed (zipped) archive files.

## Usage
- `Compress-Archive -Path path\to\file.txt -DestinationPath path\to\file.zip` : compress a single file
- `Compress-Archive -Path path\to\file1, path\to\file2, ... -DestinationPath path\to\files.zip` : compress multiple files
- `Compress-Archive -Path path\to\directory -DestinationPath path\to\directory.zip` : compress a directory
- `Compress-Archive -Path path\to\file -DestinationPath path\to\directory.zip -Update` : update an existing archive
- `Compress-Archive -Path path\to\directory -DestinationPath path\to\directory.zip -CompressionLevel Optimal|Fastest|NoCompression` : set compression level

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
archives, files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/compress-archive
