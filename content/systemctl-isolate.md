# systemctl isolate

Start the specified unit with its dependencies and stop all others. Ignores the units which have `IgnoreOnIsolate=yes`.

## Usage
- `systemctl isolate target` : switch to a target (assumed `.target` if no extension provided)
- `systemctl isolate graphical.target` : switch to graphical target explicitly
- `systemctl isolate rescue.target` : switch to rescue (single-user) mode
- `systemctl isolate emergency.target` : switch to emergency mode

## Categories
packages

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-isolate
