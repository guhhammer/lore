# blkdiscard

丢弃存储设备上的设备扇区。对 SSD 有用。

## Usage
- `blkdiscard /dev/设备名` : 丢弃设备上的所有扇区，删除所有数据
- `blkdiscard -s /dev/设备名` : 安全地丢弃设备上的所有块，删除所有数据
- `blkdiscard -l 100MB /dev/设备名` : 丢弃设备的前 100 MB

## Links
- More information: https://manned.org/blkdiscard
