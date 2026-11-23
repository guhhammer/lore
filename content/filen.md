# filen

Interface with Filen, an end-to-end encrypted cloud storage service.

## Usage
- `filen` : enter interactive mode
- `filen upload path/to/local_file remote_folder_id` : upload a local file to a specific remote folder
- `filen download remote_id path/to/local_destination` : download a file or folder using its remote ID
- `filen ls path/to/remote_folder` : list files and folders inside a remote folder
- `filen rm remote_id` : delete a remote file or folder (move it to trash)
- `filen trash restore remote_id` : restore a trashed item
- `filen sync path/to/local_folder:/path/to/remote_folder --continuous` : synchronize a local folder with a remote folder (two-way sync)
- `filen sync path/to/local_folder:ctl:/path/to/remote_folder` : download changes from the cloud to a local folder (one-way sync)

## Categories
files, network, cloud

## Links
- More information: https://github.com/FilenCloudDienste/filen-cli
- tldr page: https://tldr.inbrowser.app/pages/common/filen
