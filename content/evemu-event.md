# evemu-event

Create a virtual input device and replay an event sequence.

## Usage
- `evemu-device [description-file] evemu-play /dev/input/eventX < event-sequence evemu-play event-sequence.txt` : the general form, as the manual gives it
- `evemu-event /dev/input/eventX [--sync] --type <type> --code <code> --value <value>` : another form of the same command

## Categories
hardware
