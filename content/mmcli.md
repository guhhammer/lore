# mmcli

Control and monitor the ModemManager.

## Usage
- `mmcli -L` : list available modems
- `mmcli -M` : list available modems and monitor for ones added or removed
- `mmcli -m modem_number` : print information about a modem
- `mmcli -m modem_number -e` : enable a modem
- `sudo mmcli -m modem_number --messaging-list-sms` : list SMS messages available on the modem
- `sudo mmcli -m modem_number --messaging-delete-sms path/to/message_file` : delete a message from the modem, specifying its path

## Categories
monitoring

## Links
- More information: https://www.freedesktop.org/software/ModemManager/man/latest/mmcli.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/mmcli
