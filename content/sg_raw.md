# sg_raw

Send arbitrary SCSI command to a connected device.

## Usage
- `sg_raw /dev/sr0 EA 00 00 00 00 01` : send a command to an optical SCSI device assigned to `sr0` to load the media in its tray
- `sg_raw -i path/to/IFILE /dev/sgX scsi_command` : read data from `IFILE` instead of `stdin`
- `sg_raw -k LEN /dev/sgX scsi_command` : skip the first `LEN` bytes of input data
- `sg_raw -s SLEN /dev/sgX scsi_command` : read `SLEN` bytes of data and send to the device
- `sg_raw -t SEC /dev/sgX scsi_command` : wait up to `SEC` seconds for `sg_raw` to finish processing
- `sg_raw -v /dev/sgX scsi_command` : increase verbosity level by 1
- `sg_raw -b /dev/sgX scsi_command` : dump returned data in binary form
- `sg_raw -o path/to/OFILE /dev/sgX scsi_command` : write data received from the specified device to an `OFILE`

## Categories
hardware

## Links
- More information: https://manned.org/sg_raw
- tldr page: https://tldr.inbrowser.app/pages/common/sg_raw
