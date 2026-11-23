# pv

Monitor the progress of data through a pipe.

## Usage
- `pv path/to/file` : print the contents of the file and display a progress bar
- `command1 | pv -s expected_amount_of_data_for_eta | command2` : measure the speed and amount of data flow between pipes (`--size` is optional)
- `pv -cN in path/to/file.txt | grep pattern | pv -cN out >path/to/filtered_file.txt` : filter a file, see both progress and amount of output data
- `pv -d process_id` : attach to an already running process and see its file reading progress
- `pv -EE path/to/faulty_media >path/to/image.img` : read an erroneous file, skip errors as `dd conv=sync,noerror` would
- `pv -L 1K -S maximum_file_size_to_be_read` : stop reading after reading specified amount of data, rate limit to 1K/s
- `pv path/to/file | sha256sum` : calculate hash (MD5, SHA1, SHA256, etc.) of a large file and show progress
- `command | pv -t` : only display a timer for how long a command has taken to run

## Categories
files, monitoring

## Links
- More information: https://manned.org/pv
- tldr page: https://tldr.inbrowser.app/pages/common/pv
