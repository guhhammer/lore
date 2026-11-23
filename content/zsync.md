# zsync

Partial/differential file downloader. HTTPS is not supported - use HTTP URLs only.

## Usage
- `zsync path/to/url.zsync` : download a file using a `.zsync` control file
- `zsync -i path/to/existing_file path/to/url.zsync` : use a local file as a seed to avoid re-downloading unchanged parts
- `zsync -i path/to/existing_file -o path/to/new_file path/to/url.zsync` : save the updated file under a specific name
- `zsync -k path/to/url.zsync` : resume a partial download and keep the temporary file
- `zsync -q path/to/url.zsync` : run in quiet mode with minimal output (no progress bar, download rate, or ETA display)

## See also
- rsync

## Categories
files, network

## Links
- More information: https://manned.org/zsync
- tldr page: https://tldr.inbrowser.app/pages/linux/zsync
