# btattach

Attach serial devices to BlueZ stack.

## Usage
- `btattach [-B device] [-A device] [-P protocol] [-R]` : the general form, as the manual gives it
- `btattach -B device` : attach a BR/EDR controller
- `btattach -A device` : attach an AMP controller
- `btattach -P protocol` : specify the protocol type for talking to the device
- `btattach -S baudrate` : specify which baudrate to use

## Categories
hardware
