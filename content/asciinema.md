# asciinema

Record and replay terminal sessions, and optionally share them on <https://asciinema.org>.

## Usage
- `asciinema a` : associate the local install of `asciinema` with an asciinema.org account
- `asciinema r path/to/recording.cast` : make a new recording and save it to a local file (finish it with `<Ctrl d>` or type `exit`)
- `asciinema p path/to/recording.cast` : replay a terminal recording from a local file
- `asciinema p https://asciinema.org/a/cast_id` : replay a terminal recording hosted on <https://asciinema.org>
- `asciinema r -i 2.5` : make a new recording, limiting any idle time to at most 2.5 seconds
- `asciinema ca path/to/recording.cast` : print the full output of a locally saved recording
- `asciinema u path/to/recording.cast` : upload a locally saved terminal session to asciinema.org
- `asciinema st --local` : stream the current terminal on a local webpage

## See also
- terminalizer
- agg

## Categories
network, shell, files

## Links
- More information: https://docs.asciinema.org/manual/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/asciinema
