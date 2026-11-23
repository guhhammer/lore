# asterisk

Run and manage telephone and exchange (phone) server instances.

## Usage
- `asterisk -r -vvv` : reconnect to a running server, and turn on logging 3 levels of verbosity
- `asterisk -r -x "command"` : reconnect to a running server, run a single command, and return
- `asterisk -r -x "sip show peers"` : show chan_SIP clients (phones)
- `asterisk -r -x "core show channels"` : show active calls and channels
- `asterisk -r -x "voicemail show users"` : show voicemail mailboxes
- `asterisk -r -x "hangup request channel_id"` : terminate a channel
- `asterisk -r -x "sip reload"` : reload chan_SIP configuration

## Links
- More information: https://docs.asterisk.org/Operation/
- tldr page: https://tldr.inbrowser.app/pages/linux/asterisk
