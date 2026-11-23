# metaflac

View and edit metadata in FLAC audio files.

## Usage
- `metaflac --show-all-tags path/to/file.flac` : show all tags in a FLAC file
- `metaflac --show-tag tag_name1 --show-tag tag_name2 path/to/file.flac` : show one or more specific tags from a FLAC file
- `metaflac --set-tag "tag_name=tag_value" path/to/file.flac` : set a tag in a FLAC file
- `metaflac --remove-tag tag_name path/to/file.flac` : remove all tags with a specific name from a FLAC file
- `metaflac --remove-all-tags path/to/file.flac` : remove all tags from a FLAC file
- `metaflac --import-tags-from path/to/tags.txt path/to/file.flac` : import tags from a file (one `NAME=VALUE` pair per line)
- `metaflac --list path/to/file.flac` : list all metadata blocks in a FLAC file
- `metaflac --show-sample-rate --show-channels path/to/file.flac` : show the sample rate and number of channels from a FLAC file

## See also
- flac

## Categories
files, media

## Links
- More information: https://xiph.org/flac/documentation_tools_metaflac.html
- tldr page: https://tldr.inbrowser.app/pages/common/metaflac
