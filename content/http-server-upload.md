# http-server-upload

Zero-configuration HTTP server which provides a lightweight interface to upload files.

## Usage
- `http-server-upload` : start an HTTP server on the default port to upload files to the current directory
- `MAX_FILE_SIZE=size_in_megabytes http-server-upload` : start an HTTP server with the specified maximum allowed file size for uploads in MiB (defaults to 200 MiB)
- `PORT=port http-server-upload` : start an HTTP server on a specific port to upload files to the current directory
- `UPLOAD_DIR=path/to/directory http-server-upload` : start an HTTP server, storing the uploaded files in a specific directory
- `UPLOAD_TMP_DIR=path/to/directory http-server-upload` : start an HTTP server using a specific directory to temporarily store files during the upload process
- `TOKEN=secret http-server-upload` : start an HTTP server accepting uploads with a specific token field in the HTTP post

## Categories
network, files

## Links
- More information: https://github.com/crycode-de/http-server-upload
- tldr page: https://tldr.inbrowser.app/pages/common/http-server-upload
