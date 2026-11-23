# rage

A simple, secure, and modern file encryption tool (and Rust library) with small explicit keys, no configuration options, and UNIX-style composability. Rust implementation of `age`.

## Usage
- `echo "Your secret message" | rage --encrypt --recipient user --output path/to/message.age` : encrypt a file for `user` and save it to `message.age`
- `rage --decrypt --identity path/to/identity_file --output message` : decrypt a file with `identity_file` and save it to `message`

## Categories
files, security, development

## Links
- More information: https://github.com/str4d/rage
- tldr page: https://tldr.inbrowser.app/pages/common/rage
