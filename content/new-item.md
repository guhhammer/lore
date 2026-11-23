# new-item

Create a new file, directory, symbolic link, or a registry entry. Note: This command can only be used through PowerShell.

## Usage
- `New-Item path\to\file` : create a new blank file (equivalent to `touch`)
- `New-Item -ItemType Directory path\to\directory` : create a new directory
- `New-Item path\to\file -Value content` : write a new text file with specified content
- `New-Item path\to\file1 , path\to\file2 , ... -Value content` : write the same text file in multiple locations
- `New-Item -ItemType SymbolicLink|HardLink|Junction -Path path\to\link_file -Target path\to\source_file_or_directory` : create a symbolic link\hard link\junction to a file or directory
- `New-Item path\to\registry_key` : create a new blank registry entry (in REG_SZ, use `New-ItemProperty` or `Set-ItemProperty` to fine-tune the value type)
- `New-Item path\to\registry_key -Value value` : create a new blank registry entry with specified value

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/new-item
