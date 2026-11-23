# fs_cli

Connect to and control a running FreeSWITCH server using the FreeSWITCH Command Line Interface (ESL client).

## Usage
- `fs_cli` : connect to the local FreeSWITCH instance with an interactive session
- `fs_cli -H host -P port -p password` : connect to a remote FreeSWITCH server
- `fs_cli -x "command"` : execute a single FreeSWITCH command and exit
- `fs_cli -x "status"` : show FreeSWITCH system status
- `fs_cli -x "reloadxml"` : reload FreeSWITCH XML configuration
- `fs_cli -x "module_exists module_name"` : check if a module is loaded
- `fs_cli -x "show calls"` : show active calls
- `fs_cli -r` : retry connection on failure

## Categories
network, text, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/fs_cli
