# pcilmr

Margin PCIe Links.

## Usage
- `pcilmr [--margin] [<common options>] <link port> [<link options>] [<link port> [<link options>] ...]` : the general form, as the manual gives it
- `pcilmr --full [<common options>]` : another form of the same command
- `pcilmr --scan` : another form of the same command
- `pcilmr --scan Scan` : for  Links with negotiated speed 16 GT/s or higher
- `pcilmr -e <errors>` : specify Error Count Limit for margining
- `pcilmr -d <time>` : specify dwell time in seconds for the margining step
- `pcilmr -p <parallel_lanes>` : specify number of lanes to margin simultaneously

## Categories
files
