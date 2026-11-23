# msg

Send a message to a user or session.

## Usage
- `msg username|session_name|session_id message` : send a message to a specified user or session
- `echo "message" | msg username|session_name|session_id` : send a message from `stdin`
- `msg /server:server_name username|session_name|session_id` : send a message to a specific server
- `msg *` : send a message to all users of the current machine
- `msg /time:10` : set a delay in seconds for a message

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
users, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/msg
