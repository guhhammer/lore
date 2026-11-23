# treemd

View markdown files with tree-based navigation and an interactive TUI.

## Usage
- `treemd path/to/file.md` : open a markdown file in interactive TUI mode
- `treemd -l path/to/file.md` : list all headings in a markdown file
- `treemd --tree path/to/file.md` : show the heading tree structure of a markdown file
- `treemd -s heading_name path/to/file.md` : extract a specific section by heading name
- `treemd -l --filter pattern path/to/file.md` : filter headings by a specific pattern
- `treemd -q '.h2 | text' path/to/file.md` : query and extract markdown structure using the treemd query language

## Categories
files, documents, text

## Links
- More information: https://github.com/Epistates/treemd#usage
- tldr page: https://tldr.inbrowser.app/pages/common/treemd
