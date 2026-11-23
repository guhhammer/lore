# ifne

Run a command depending on the emptiness of `stdin`.

## Usage
- `ifne command command_options` : run the specified command if and only if `stdin` is not empty
- `ifne -n command command_options` : run the specified command if and only if `stdin` is empty, otherwise pass `stdin` to `stdout`

## Links
- More information: https://manned.org/ifne
- tldr page: https://tldr.inbrowser.app/pages/common/ifne
