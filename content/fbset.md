# fbset

Show and modify frame buffer device settings.

## Usage
- `sudo fbset -i` : show current framebuffer settings
- `sudo fbset "800x600-60"` : set a framebuffer mode defined in `/etc/fb.modes`
- `sudo fbset -xres horizontal_pixels -yres vertical_pixels` : set the dimensions of the framebuffer
- `sudo fbset -g tty_horizontal tty_vertical monitor_horizontal monitor_vertical color_depth` : set an arbitrary framebuffer mode

## Categories
users, hardware

## Links
- More information: https://manned.org/fbset.1
- tldr page: https://tldr.inbrowser.app/pages/linux/fbset
