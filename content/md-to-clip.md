# md-to-clip

Convert tldr-pages to Command Line Interface Pages.

## Usage
- `md-to-clip path/to/page1.md path/to/page2.md ...` : convert tldr-pages files and save into the same directories
- `md-to-clip --output-directory path/to/directory path/to/page1.md path/to/page2.md ...` : convert tldr-pages files and save into a specific directory
- `md-to-clip --no-file-save <(echo 'page_content')` : convert a tldr-page file to `stdout`
- `md-to-clip --special-placeholder-config path/to/config.yaml path/to/page1.md path/to/page2.md ...` : convert tldr-pages files while recognizing additional placeholders from a specific config
- `md-to-clip --help` : display help
- `md-to-clip --version` : display version

## See also
- clip-view

## Categories
files, network, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/md-to-clip
