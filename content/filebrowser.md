# filebrowser

Simple HTTP web server to manage files and directories.

## Usage
- `filebrowser` : start a new server instance serving the current directory
- `filebrowser -r path/to/directory` : start a new server instance serving a specific root directory
- `filebrowser -a host -p port -r path/to/directory` : start an instance with different host address (defaults to `127.0.0.1`) and port (defaults to `8080`)
- `filebrowser -c path/to/file -d path/to/database.db -r path/to/directory` : start an instance with a specified configuration file, storing the application database in a specific location (defaults to `filebrowser.db` on the current directory)
- `filebrowser --username username --password password -r path/to/directory` : set up a different default first-time account username and password (both default to `admin`) when setting up a new instance
- `filebrowser --img-processors 4 -r path/to/directory` : set up the maximum amount of image processors used when generating thumbnails (defaults to `4`)
- `filebrowser --disable-exec --disable-thumbnails -r path/to/directory` : disable image thumbnails as well as the Command Runner feature, limiting access for hosted script files from being executed inside the app
- `filebrowser --disable-preview-resize --disable-type-detection-by-header -r path/to/directory` : disable resizing of image previews as well as detecting file types by reading their headers

## Categories
files, network, users

## Links
- More information: https://filebrowser.org/cli/filebrowser.html
- tldr page: https://tldr.inbrowser.app/pages/common/filebrowser
