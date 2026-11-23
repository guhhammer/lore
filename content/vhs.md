# vhs

Generate terminal GIFs from a tape file.

## Usage
- `vhs new path/to/file.tape` : create a tape file (add commands to the tape file using an editor)
- `vhs record >path/to/file.tape` : record inputs to a tape file
- `vhs record -s shell >path/to/file.tape` : record inputs to a tape file using a specific shell
- `exit` : stop recording
- `vhs validate path/to/file.tape` : validate the syntax of a tape file
- `vhs path/to/file.tape` : create a GIF from a tape file
- `vhs publish path/to/file.gif` : publish a GIF to <https://vhs.charm.sh> and get a shareable URL

## Categories
files, shell, media

## Links
- More information: https://github.com/charmbracelet/vhs
- tldr page: https://tldr.inbrowser.app/pages/common/vhs
