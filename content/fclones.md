# fclones

Efficient duplicate file finder and remover.

## Usage
- `fclones group .` : search for duplicate files in the current directory
- `fclones group --cache path/to/directory1 path/to/directory2 ...` : search multiple directories for duplicate files and cache the results
- `fclones group path/to/directory --depth 1 >path/to/file.txt` : search only the specified directory for duplicate files, skipping subdirectories, and save the results into a file
- `fclones <path/to/file.txt move path/to/target_directory` : move the duplicate files in a TXT file to a different directory
- `fclones <path/to/file.txt link --soft --dry-run 2 >/dev/null` : simulate soft links in a TXT file without actually linking
- `fclones group . | fclones remove --priority newest` : delete the newest duplicates from the current directory without storing them in a file
- `fclones group . --name '*.jpg' -i --transform 'exiv2 -d a $IN' --in-place` : preprocess JPEG files in the current directory by using an external command to strip their EXIF data before matching for duplicates

## Categories
files, users

## Links
- More information: https://github.com/pkolaczk/fclones#usage
- tldr page: https://tldr.inbrowser.app/pages/common/fclones
