# rauc

Lightweight update client for Embedded Linux devices to create, inspect, and install update bundles.

## Usage
- `rauc status` : display the current system status and slot overview
- `rauc install path/to/bundle.raucb|https://example.com/bundle.raucb` : install an update bundle
- `rauc info path/to/bundle.raucb` : show information about a specific update bundle
- `rauc bundle --cert path/to/cert.pem --key path/to/key.pem path/to/input_dir path/to/output.raucb` : create a signed update bundle from a directory
- `rauc extract path/to/bundle.raucb path/to/output_directory` : extract the contents of an update bundle
- `rauc status mark-good` : mark the currently booted slot as good to confirm successful update
- `rauc status mark-bad slot_name` : mark a specific slot as bad to prevent it from booting
- `rauc status mark-active other` : switch the active boot slot to the other available slot

## Categories
files, packages, hardware

## Links
- More information: https://manned.org/rauc
- tldr page: https://tldr.inbrowser.app/pages/linux/rauc
