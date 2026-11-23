# chat

Automate conversations with a modem or serial device. Commonly used to establish PPP (Point-to-Point Protocol) connections.

## Usage
- `chat 'expect_send_pairs'` : execute a chat script directly
- `chat -f 'path/to/chat_script'` : execute a chat script from a file
- `chat -t timeout_in_seconds 'expect_send_pairs'` : set a custom timeout (in seconds) for expecting a response
- `chat -v 'expect_send_pairs'` : enable verbose output to log the conversation to `syslog`
- `chat -r path/to/report_file 'expect_send_pairs'` : use a report file to log specific strings received during the conversation
- `chat -T 'phone_number' '"ATDT\\T CONNECT"'` : dial a phone number using a variable, substituting `\T` in the script
- `chat 'ABORT "error_string" expect_send_pairs'` : include an abort condition if a specific string is received

## Categories
files, development, hardware

## Links
- More information: https://manned.org/chat
- tldr page: https://tldr.inbrowser.app/pages/linux/chat
