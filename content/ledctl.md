# ledctl

Intel(R) Enclosure LED Control Application.

## Usage
- `sudo ledctl locate=/dev/sda,/dev/sdb,...` : turn on the "Locate" LED for specified device(s)
- `sudo ledctl locate_off=/dev/sda,/dev/sdb,...` : turn off the "Locate" LED for specified device(s)
- `sudo ledctl off=/dev/sda,/dev/sdb,...` : turn off the "Status" LED and "Failure" LED for specified device(s)
- `sudo ledctl normal=/dev/sda,/dev/sdb,...` : turn off the "Status" LED, "Failure" LED, and "Locate" LED for specified device(s)

## Categories
hardware, users

## Links
- More information: https://manned.org/ledctl
- tldr page: https://tldr.inbrowser.app/pages/linux/ledctl
