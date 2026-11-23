# apkleaks

Expose URIs, endpoints, and secrets from APK files. Note: APKLeaks utilizes the `jadx` disassembler to decompile APK files.

## Usage
- `apkleaks -f path/to/file.apk` : scan an APK file for URIs, endpoints, and secrets
- `apkleaks -f path/to/file.apk -o path/to/output.txt` : scan and save the output to a specific file
- `apkleaks -f path/to/file.apk -a "--threads-count 5 --deobf"` : pass `jadx` disassembler arguments

## Categories
files

## Links
- More information: https://github.com/dwisiswant0/apkleaks
- tldr page: https://tldr.inbrowser.app/pages/common/apkleaks
