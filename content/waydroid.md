# waydroid

A container-based approach to boot a full Android system on a regular Linux system like Ubuntu.

## Usage
- `waydroid show-full-ui` : start Waydroid
- `sudo waydroid init` : initialize Waydroid (required on first run or after reinstalling Android)
- `waydroid status` : check the container status
- `waydroid app install path/to/file.apk` : install a new Android app from a file
- `waydroid session start|stop` : start or stop the Waydroid session
- `sudo waydroid container start|stop|restart|freeze|unfreeze` : manage the Waydroid container
- `sudo waydroid shell` : open Waydroid shell
- `waydroid prop set persist.waydroid.width|height number` : adjust Waydroid window dimensions

## Categories
containers, files, system

## Links
- More information: https://docs.waydro.id/usage/waydroid-command-line-options
- tldr page: https://tldr.inbrowser.app/pages/linux/waydroid
