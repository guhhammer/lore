# gdown

Download files from Google Drive and other URLs.

## Usage
- `gdown url` : download a file from a URL
- `gdown file_id` : download using a file ID
- `gdown --fuzzy url` : download with fuzzy file ID extraction (also works with <https://docs.google.com> links)
- `gdown folder_id|url -O path/to/output_directory --folder` : download a folder using its ID or the full URL
- `gdown tar_url -O - -q | tar xvf -` : download a `.tar` archive, write it to `stdout`, and extract it

## Categories
network, files, archives

## Links
- More information: https://github.com/wkentaro/gdown#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gdown
