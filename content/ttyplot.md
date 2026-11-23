# ttyplot

A realtime plotting utility for the command-line with data input from `stdin`.

## Usage
- `{ echo 1 2 3; cat; } | ttyplot` : plot the values `1`, `2`, and `3` (Note: `cat` prevents `ttyplot` from exiting)
- `{ echo 1 2 3; cat; } | ttyplot -t title -u unit` : set a specific title and unit
- `{ while true; do echo $RANDOM; sleep 1; done } | ttyplot` : use a while loop to continuously plot random values
- `ping 8.8.8.8 | sed -u 's/^.*time=//g; s/ ms//g' | ttyplot -t "ping to 8.8.8.8" -u ms` : parse the output from `ping` and visualize it

## Categories
network, text

## Links
- More information: https://github.com/tenox7/ttyplot
- tldr page: https://tldr.inbrowser.app/pages/linux/ttyplot
