# rtl_sdr

Raw data recorder for RTL-SDR receivers. Data is encoded using I/Q sampling (aka quadrature sampling).

## Usage
- `rtl_sdr -f 100000000 path/to/file` : save RAW data from a frequency (specified in Hz) to a file
- `rtl_sdr -f 100000000 - | aplay` : pipe data to another program
- `rtl_sdr -f 100000000 -n 20 -` : read a specified number of samples
- `rtl_sdr -f 100000000 -s 2400000 -` : specify the sample rate in Hz (ranges 225001-300000 and 900001-3200000)
- `rtl_sdr -f 100000000 -d 0 -` : specify the device by its index
- `rtl_sdr -f 100000000 -g 20 -` : specify the gain
- `rtl_sdr -f 100000000 -b 9999999 -` : specify the output block size
- `rtl_sdr -f 100000000 -S -` : use synchronous output

## Categories
files

## Links
- More information: https://osmocom.org/projects/rtl-sdr/wiki/Rtl-sdr
- tldr page: https://tldr.inbrowser.app/pages/common/rtl_sdr
