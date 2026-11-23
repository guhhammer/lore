# gdrive

Interact with Google Drive. Folder/file ID can be obtained from the Google Drive folder or ID URL.

## Usage
- `gdrive upload -p folder_id path/to/file_or_folder` : upload a local path to the parent folder with the specified ID
- `gdrive download file_or_directory_id` : download file or directory by ID to current directory
- `gdrive download --path path/to/folder file_or_directory_id` : download to a given local path by its ID
- `gdrive update file_or_folder_id path/to/file_or_folder` : create a new revision of an ID using a given file or folder

## Categories
files, network, disks

## Links
- More information: https://github.com/prasmussen/gdrive
- tldr page: https://tldr.inbrowser.app/pages/common/gdrive
