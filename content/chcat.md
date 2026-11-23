# chcat

Change SELinux security category for files. Categories provide an additional level of access control based on MCS (Multi-Category Security).

## Usage
- `sudo chcat -L` : list all available categories
- `sudo chcat +CategoryName path/to/file` : add a category to a file
- `sudo chcat -- -CategoryName path/to/file` : remove a category from a file
- `sudo chcat CategoryName1,CategoryName2,... path/to/file` : set specific categories for a file (replacing existing ones)
- `ls -Z path/to/file` : display the categories of a file
- `sudo chcat -d path/to/file` : remove all categories from a file

## See also
- chcon
- semanage

## Categories
files, users

## Links
- More information: https://manned.org/chcat
- tldr page: https://tldr.inbrowser.app/pages/linux/chcat
