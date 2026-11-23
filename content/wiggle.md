# wiggle

A patch application tool resolving conflicts in patches that `patch` cannot handle. Note: Wiggle forcefully applies all changes, merging when conflicts arise, and reporting unresolvable issues.

## Usage
- `wiggle path/to/file.patch` : apply changes from the patch file to the original file
- `wiggle path/to/file.patch -o path/to/output_file.txt` : apply changes to the output file
- `wiggle -r path/to/file path/to/file.rej` : take any changes in `file.rej` that could not have been applied and merge them into a file
- `wiggle -x path/to/file.patch` : extract one branch of a patch or merge file
- `wiggle -w path/to/my_word_patch.patch -o path/to/word_patched_code.c` : apply a patch and save the compared words to the output file
- `wiggle -m -h` : display help about the merge function

## Categories
files, version-control

## Links
- More information: https://manned.org/wiggle
- tldr page: https://tldr.inbrowser.app/pages/common/wiggle
