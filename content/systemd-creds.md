# systemd-creds

List, show, encrypt, and decrypt service credentials.

## Usage
- `systemd-creds encrypt --name name path/to/input_file path/to/output` : encrypt a file and set a specific name
- `systemd-creds decrypt path/to/input_file path/to/output_file` : decrypt the file again
- `echo -n text | systemd-creds encrypt --name name - path/to/output` : encrypt text from `stdin`
- `echo -n text | systemd-creds encrypt --name name --pretty - - >>service` : encrypt the text and append it to the `.service` file (the credentials will be available in `$CREDENTIALS_DIRECTORY`)
- `systemd-creds encrypt --not-after "timestamp" path/to/input_file path/to/output_file` : create a credential that is only valid until the given timestamp

## Categories
system, security, files

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-creds.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-creds
