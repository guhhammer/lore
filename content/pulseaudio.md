# pulseaudio

The PulseAudio sound system daemon and manager.

## Usage
- `pulseaudio --check` : check if PulseAudio is running (a non-zero exit code means it is not running)
- `pulseaudio --start` : start the PulseAudio daemon in the background
- `pulseaudio -k` : kill the running PulseAudio daemon
- `pulseaudio --dump-modules` : list available modules
- `pulseaudio -L "module_name arguments"` : load a module into the currently running daemon with the specified arguments

## Categories
system, media

## Links
- More information: https://manned.org/pulseaudio
- tldr page: https://tldr.inbrowser.app/pages/linux/pulseaudio
