# slurp

Select a region in a Wayland compositor.

## Usage
- `slurp` : select a region and print it to `stdout`
- `slurp -d` : select a region and print it to `stdout`, while displaying the dimensions of the selection
- `slurp -p` : select a single point instead of a region
- `slurp -o -f '%o'` : select an output and print its name
- `grim -g "$(slurp -w 0)"` : select a specific region and take a borderless screenshot of it, using `grim`
- `wf-recorder -g "$(slurp -w 0)"` : select a specific region and take a borderless video of it, using `wf-recorder`

## Categories
desktop, documents

## Links
- More information: https://github.com/emersion/slurp/blob/master/slurp.1.scd
- tldr page: https://tldr.inbrowser.app/pages/linux/slurp
