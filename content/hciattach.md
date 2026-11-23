# hciattach

Attach serial devices via UART HCI to BlueZ stack.

## Usage
- `hciattach [OPTIONS] <tty> <type|id> [speed] [flow] [sleep] [bdaddr]` : the general form, as the manual gives it
- `hciattach -1` : another form of the same command
- `hciattach -i` : send break
- `hciattach -n` : don't detach from controlling terminal
- `hciattach -p` : print the PID when detaching
- `hciattach -t timeout` : specify an initialization timeout

## Categories
hardware
